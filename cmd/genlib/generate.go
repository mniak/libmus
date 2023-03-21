package main

import (
	"embed"
	"fmt"
	"strings"
	"text/template"

	"github.com/iancoleman/strcase"
	"github.com/mniak/libmus/cmd/models"
	"github.com/samber/lo"
)

//go:embed templates/*.tmpl
var efs embed.FS

type O = map[string]any

const packageName = "main"

func GenerateBindings(pkg *models.Package) (map[string]string, error) {
	tmpl, err := template.ParseFS(efs, "templates/*.tmpl")
	if err != nil {
		return nil, err
	}

	result := make(map[string]string)
	for _, st := range pkg.Structs {
		var sb strings.Builder
		tmpl.ExecuteTemplate(&sb, "file.go.tmpl", O{
			"PackageName":    packageName,
			"LibraryPackage": "github.com/mniak/libmus",
			"Struct":         st,
			"Functions": lo.Map(st.Functions, func(fn models.Function, _ int) O {
				return O{
					"StructName":   st.Name,
					"FunctionName": fn.Name,
					"ReturnType":   getReturnTypeForTemplate(fn.ReturnType),
				}
			}),
		})

		filename := fmt.Sprintf("%s.go", snakify(st.Name))
		result[filename] = sb.String()
	}

	return result, nil
}

func getReturnTypeForTemplate(returnType string) O {
	switch returnType {
	case "string":
		fallthrough
	default:
		return O{
			"CType": "*C.char",
			"GoToC": "C.CString(%s)",
		}
	}
}

func snakify(str string) string {
	str = strcase.ToSnake(str)
	str = strings.ReplaceAll(str, "pitch_class", "pitchclass")
	return str
}
