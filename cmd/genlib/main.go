package main

import (
	"fmt"
	"go/ast"
	"log"
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

	files, err := GenerateBindings(pkg)
	if err != nil {
		log.Fatalln(err)
	}
	for fname, fcontent := range files {
		file, err := os.Create(filepath.Join("clib", fname))
		if err != nil {
			log.Fatalln(err)
		}
		defer file.Close()

		fmt.Fprintln(file, fcontent)
	}
}

func ConvertPackage(packageName string) *models.Package {
	pkg := models.NewPackage()
	visitPackage(packageName,
		VisitorFunctions{
			OnFuncDecl: func(fd *ast.FuncDecl) {
				fn, err := models.ParseFunction(fd)
				if err != nil {
					log.Fatalln(err)
				}
				if fn.Struct != nil {
					pkg.Struct(fn.Struct.Name).AppendMethod(fn)
				} else {
					pkg.AppendFunction(fn)
				}
			},
		},
	)
	return pkg
}
