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
		}, genMethods(st)...),
		Scope: &ast.Scope{
			Objects: map[string]*ast.Object{},
		},
	}

	err := format.Node(&sb, fset, file)
	return sb.String(), err
}

func genMethods(st *models.Struct) []ast.Decl {
	return lo.Map(st.Functions, func(fn models.Function, _ int) ast.Decl {
		fd := &ast.FuncDecl{}
		fd.Type = &ast.FuncType{
			Params: &ast.FieldList{
				List: []*ast.Field{},
			},
			Results: &ast.FieldList{
				List: []*ast.Field{},
			},
		}

		g := FuncGenerator{
			initialVar:     "a",
			statements:     []ast.Stmt{},
			function:       fn,
			structTypeInfo: getTypeInfo(fn.Struct),
			returnTypeInfo: getTypeInfo(fn.Return),
		}
		if fn.Constructor {
			fd.Name = ast.NewIdent(fn.Name)
		} else {
			fd.Name = ast.NewIdent(fmt.Sprintf("%s_%s", st.Name, fn.Name))
			fd.Type.Params.List = append(fd.Type.Params.List, &ast.Field{
				Names: []*ast.Ident{
					ast.NewIdent(g.nextVar()),
				},
				Type: ast.NewIdent(g.structTypeInfo.CType),
			})
		}

		fd.Doc = &ast.CommentGroup{
			List: []*ast.Comment{
				{
					Text: fmt.Sprintf("//export %s", fd.Name),
				},
			},
		}

		fd.Body = g.genMethodBody()

		fd.Type.Params.List = append(fd.Type.Params.List, lo.Map(fn.Parameters, func(param models.Parameter, _ int) *ast.Field {
			paramTypeInfo := getTypeInfo(param.Type)

			return &ast.Field{
				Names: []*ast.Ident{
					ast.NewIdent(param.Name),
				},
				Type: ast.NewIdent(paramTypeInfo.CType),
			}
		})...)

		if g.returnTypeInfo != nil {
			fd.Type.Results.List = append(fd.Type.Results.List, &ast.Field{
				Type: ast.NewIdent(g.returnTypeInfo.CType),
			})
		}
		return fd
	})
}

func (g *FuncGenerator) genFunctionCall() ast.Expr {
	sel := ast.SelectorExpr{
		Sel: ast.NewIdent(g.function.Name),
	}

	if g.function.Constructor {
		sel.X = ast.NewIdent(libName)
	} else {
		sel.X = ast.NewIdent(g.lastVar())
	}

	fnCall := ast.CallExpr{
		Fun: &sel,
		Args: lo.Map(g.function.Parameters, func(param models.Parameter, _ int) ast.Expr {
			paramTypeInfo := getTypeInfo(param.Type)
			return paramTypeInfo.CToGo.Convert(ast.NewIdent(param.Name))
		}),
	}
	return &fnCall
}

type FuncGenerator struct {
	initialVar     string
	vars           []string
	statements     []ast.Stmt
	function       models.Function
	structTypeInfo *TypeInfo
	returnTypeInfo *TypeInfo
}

func (g *FuncGenerator) lastVar() string {
	return g.vars[len(g.vars)-1]
}

func (g *FuncGenerator) nextVar() string {
	const initialVarFallback = "h"
	if len(g.vars) == 0 {
		if g.initialVar == "" {
			g.vars = append(g.vars, initialVarFallback)
		} else {
			g.vars = append(g.vars, g.initialVar)
		}
	} else {
		g.vars = append(g.vars, g.lastVar()+"a")
	}

	return g.lastVar()
}

func (g *FuncGenerator) addConversion(conv Conversion) {
	if conv.IsTrivial {
		return
	}
	g.statements = append(g.statements, &ast.AssignStmt{
		Tok: token.DEFINE,
		Rhs: []ast.Expr{
			conv.Convert(ast.NewIdent(g.lastVar())),
		},
		Lhs: []ast.Expr{
			ast.NewIdent(g.nextVar()),
		},
	})
}

func (g *FuncGenerator) addFunctionCall() {
	fnCall := g.genFunctionCall()
	if g.returnTypeInfo != nil {
		g.addAssignment(fnCall)
		return
	}
	g.statements = append(g.statements, &ast.ExprStmt{fnCall})
}

func (g *FuncGenerator) addAssignment(expr ast.Expr) {
	g.statements = append(g.statements,
		&ast.AssignStmt{
			Lhs: []ast.Expr{ast.NewIdent(g.nextVar())},
			Tok: token.DEFINE,
			Rhs: []ast.Expr{expr},
		},
	)
}

func (g *FuncGenerator) addReturn() {
	if g.returnTypeInfo == nil {
		return
	}
	g.addConversion(g.returnTypeInfo.GoToC)
	g.statements = append(g.statements,
		&ast.ReturnStmt{
			Results: []ast.Expr{
				ast.NewIdent(g.lastVar()),
			},
		},
	)
}

func (g *FuncGenerator) genMethodBody() *ast.BlockStmt {
	if !g.function.Constructor {
		g.addConversion(g.structTypeInfo.CToGo)
	}
	g.addFunctionCall()
	g.addReturn()

	return &ast.BlockStmt{
		List: g.statements,
	}
}

func snakify(str string) string {
	str = strcase.ToSnake(str)
	str = strings.ReplaceAll(str, "pitch_class", "pitchclass")
	return str
}
