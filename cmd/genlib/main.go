package main

import (
	"fmt"
	"go/ast"
	"os"
	"path/filepath"

	"github.com/mniak/libmus/cmd/models"
	"github.com/samber/lo"
	"golang.org/x/tools/go/packages"
)

type VisitorFunctions struct {
	OnTypeSpec func(*ast.TypeSpec)
	OnFuncDecl func(*ast.FuncDecl)
}

func visitPackage(pkgname string, fns VisitorFunctions) {
	config := &packages.Config{
		Mode: packages.NeedSyntax,
	}
	pkgs := lo.Must(packages.Load(config, pkgname))
	pkg := pkgs[0]

	for _, s := range pkg.Syntax {
		ast.FileExports(s)
		ast.Inspect(s, func(n ast.Node) bool {
			switch node := n.(type) {
			case *ast.TypeSpec:
				if fns.OnTypeSpec != nil {
					fns.OnTypeSpec(node)
				}
			case *ast.FuncDecl:
				if fns.OnFuncDecl != nil {
					fns.OnFuncDecl(node)
				}
			}
			return true
		})
	}
}

func main() {
	pkg := ConvertPackage("github.com/mniak/libmus")

	files := lo.Must(GenerateBindings(pkg))
	for fname, fcontent := range files {
		file := lo.Must(os.Create(filepath.Join("../../clib", fname)))
		defer file.Close()

		fmt.Fprintln(file, fcontent)
	}
	// yenc := yaml.NewEncoder(os.Stdout)
	// defer yenc.Close()
	// yenc.SetIndent(2)
	// lo.Must0(yenc.Encode(pkg))
}

func ConvertPackage(packageName string) *models.Package {
	pkg := models.NewPackage()
	visitPackage(packageName,
		VisitorFunctions{
			// OnTypeSpec: func(ts *ast.TypeSpec) {
			// 	conv.Struct()
			// 	fmt.Printf("typedef void* %s;\n", ts.Name)
			// 	ast.Print(fset, ts)
			// },
			OnFuncDecl: func(fd *ast.FuncDecl) {
				fn := models.ParseFunction(fd)
				if fn.Struct != "" {
					pkg.Struct(fn.Struct).AppendMethod(fn)
				} else {
					pkg.AppendFunction(fn)
				}
			},
		},
	)
	return pkg
}
