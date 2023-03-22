package main

import (
	"bytes"
	"embed"
	"fmt"
	"go/ast"
	"go/format"
	"go/token"
	"strings"

	"github.com/iancoleman/strcase"
	"github.com/mniak/libmus/cmd/models"
	"github.com/samber/lo"
)

//go:embed templates/*.tmpl
var efs embed.FS

const (
	packageName = "main"
	libPackage  = "github.com/mniak/libmus"
	libName     = "libmus"
)

func GenerateBindings(pkg *models.Package) (map[string]string, error) {
	result := make(map[string]string)
	for _, st := range pkg.Structs {
		filename := fmt.Sprintf("%s.go", snakify(st.Name))
		text, err := renderStruct(st)
		if err != nil {
			return nil, err
		}

		result[filename] = text
	}

	return result, nil
}

func renderStruct(st *models.Struct) (string, error) {
	var sb bytes.Buffer
	fset := token.NewFileSet()

	file := &ast.File{
		Doc: &ast.CommentGroup{
			List: []*ast.Comment{
				{
					Text: "//go:build clib",
				},
			},
		},
		Name: ast.NewIdent(" " + packageName),
		Decls: append([]ast.Decl{
			&ast.GenDecl{
				Tok: token.IMPORT,
				Specs: []ast.Spec{
					&ast.ImportSpec{
						Path: &ast.BasicLit{
							Kind:  token.STRING,
							Value: fmt.Sprintf("%q", "runtime/cgo"),
						},
					},
					&ast.ImportSpec{
						Path: &ast.BasicLit{
							Kind:  token.STRING,
							Value: fmt.Sprintf("%q", libPackage),
						},
					},
				},
			},
			&ast.GenDecl{
				Doc: &ast.CommentGroup{
					List: []*ast.Comment{
						{
							Text: fmt.Sprintf("// typedef void* %s;", st.Name),
						},
					},
				},
				Tok: token.IMPORT,
				Specs: []ast.Spec{
					&ast.ImportSpec{
						Path: &ast.BasicLit{
							Kind:  token.STRING,
							Value: fmt.Sprintf("%q", "C"),
						},
					},
				},
			},
		}, genASTMethods(st)...),
		Scope: &ast.Scope{
			Objects: map[string]*ast.Object{},
		},
	}

	err := format.Node(&sb, fset, file)
	return sb.String(), err
}

func genASTMethods(st *models.Struct) []ast.Decl {
	return lo.Map(st.Functions, func(fn models.Function, _ int) ast.Decl {
		structTypeInfo := getTypeInfo(fn.Struct)
		returnTypeInfo := getTypeInfo(fn.Return)

		fd := &ast.FuncDecl{}
		fd.Type = &ast.FuncType{
			Params: &ast.FieldList{
				List: []*ast.Field{},
			},
			Results: &ast.FieldList{
				List: []*ast.Field{},
			},
		}
		fd.Body = &ast.BlockStmt{
			List: []ast.Stmt{},
		}

		var innerFuncPrefix string
		if fn.Constructor {
			fd.Name = ast.NewIdent(fn.Name)
			innerFuncPrefix = libName
		} else {
			fd.Name = ast.NewIdent(fmt.Sprintf("%s_%s", st.Name, fn.Name))

			fd.Type.Params.List = append(fd.Type.Params.List, &ast.Field{
				Names: []*ast.Ident{
					ast.NewIdent("h"),
				},
				Type: ast.NewIdent(structTypeInfo.CType),
			})
			innerFuncPrefix = "this"
			fd.Body.List = append(fd.Body.List, &ast.AssignStmt{
				Lhs: []ast.Expr{
					ast.NewIdent("this"),
				},
				Tok: token.DEFINE,
				Rhs: []ast.Expr{
					structTypeInfo.CToGo(ast.NewIdent("h")),
				},
			})
		}

		fd.Type.Params.List = append(fd.Type.Params.List, lo.Map(fn.Parameters, func(param models.Parameter, _ int) *ast.Field {
			paramTypeInfo := getTypeInfo(&param.Type)

			return &ast.Field{
				Names: []*ast.Ident{
					ast.NewIdent(param.Name),
				},
				Type: ast.NewIdent(paramTypeInfo.CType),
			}
		})...)

		fd.Doc = &ast.CommentGroup{
			List: []*ast.Comment{
				{
					Text: fmt.Sprintf("//export %s", fd.Name),
				},
			},
		}

		fnCall := &ast.CallExpr{
			Fun: ast.NewIdent(fmt.Sprintf("%s.%s", innerFuncPrefix, fn.Name)),
			Args: lo.Map(fn.Parameters, func(param models.Parameter, _ int) ast.Expr {
				paramTypeInfo := getTypeInfo(&param.Type)
				return paramTypeInfo.CToGo(ast.NewIdent(param.Name))
			}),
		}

		if returnTypeInfo != nil {
			fd.Type.Results.List = append(fd.Type.Results.List, &ast.Field{
				Type: ast.NewIdent(returnTypeInfo.CType),
			})

			fd.Body.List = append(fd.Body.List,
				&ast.ReturnStmt{
					Results: []ast.Expr{
						returnTypeInfo.GoToC(fnCall),
					},
				},
			)

		} else {
			fd.Body.List = append(fd.Body.List, &ast.ExprStmt{
				X: fnCall,
			})
		}
		return fd
	})
}

type TypeInfo struct {
	CType string
	CToGo func(ast.Expr) ast.Expr
	GoToC func(ast.Expr) ast.Expr
}

func getTypeInfo(t *models.Type) *TypeInfo {
	switch {
	case t == nil:
		return nil
	case t.IsStruct():
		return &TypeInfo{
			CType: fmt.Sprintf("C.%s", t.Name),
			CToGo: func(e ast.Expr) ast.Expr {
				return &ast.TypeAssertExpr{
					X: &ast.SelectorExpr{
						X: &ast.CallExpr{
							Fun:  ast.NewIdent("cgo.Handle"),
							Args: []ast.Expr{e},
						},
						Sel: ast.NewIdent("Value()"),
					},
					Type: ast.NewIdent(fmt.Sprintf("*%s.%s", libName, t.Name)),
				}
			},
			GoToC: func(e ast.Expr) ast.Expr {
				return &ast.CallExpr{
					Fun: ast.NewIdent(fmt.Sprintf("C.%s", t.Name)),
					Args: []ast.Expr{
						&ast.CallExpr{
							Fun:  ast.NewIdent("cgo.NewHandle"),
							Args: []ast.Expr{e},
						},
					},
				}
			},
		}
	case t.Name == "string":
		return &TypeInfo{
			CType: "*C.char",
			CToGo: func(e ast.Expr) ast.Expr {
				return &ast.CallExpr{
					Fun:  ast.NewIdent("C.GoString"),
					Args: []ast.Expr{e},
				}
			},
			GoToC: func(e ast.Expr) ast.Expr {
				return &ast.CallExpr{
					Fun:  ast.NewIdent("C.CString"),
					Args: []ast.Expr{e},
				}
			},
		}
	default:
		return &TypeInfo{
			CType: t.Name,
			CToGo: func(e ast.Expr) ast.Expr { return e },
			GoToC: func(e ast.Expr) ast.Expr { return e },
		}
	}
}

func snakify(str string) string {
	str = strcase.ToSnake(str)
	str = strings.ReplaceAll(str, "pitch_class", "pitchclass")
	return str
}
