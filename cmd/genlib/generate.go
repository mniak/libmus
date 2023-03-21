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
		err := tmpl.ExecuteTemplate(&sb, "file.go.tmpl", O{
			"PackageName":    packageName,
			"LibraryPackage": "github.com/mniak/libmus",
			"Struct":         st,
			"Functions": lo.Map(st.Functions, func(fn models.Function, _ int) O {
				return O{
					"StructType":   getTypeForTemplate(*fn.Struct),
					"FunctionName": fn.Name,
					"ReturnType":   getTypeForTemplate(fn.Return),
					"Constructor":  fn.Constructor,
				}
			}),
		})
		if err != nil {
			return nil, err
		}

		filename := fmt.Sprintf("%s.go", snakify(st.Name))
		result[filename] = sb.String()
	}

	return result, nil
}

func getTypeForTemplate(t models.Type) O {
	if t.IsStruct() {
		return O{
			"Name":  t.Name,
			"CType": fmt.Sprintf("C.%s", t.Name),
			"GoToC": fmt.Sprintf("C.%s(cgo.NewHandle(%%s))", t.Name),
			"CToGo": fmt.Sprintf("cgo.Handle(%%s).Value().(*libmus.%s)", t.Name),
		}
	}

	switch t.Name {
	case "string":
		return O{
			"CType": "*C.char",
			"GoToC": "C.CString(%s)",
			"CToGo": "%s %s",
		}
	default:
		return O{
			"CType": t.Name,
			"GoToC": "%s",
			"CToGo": "%s",
		}
	}
}

func snakify(str string) string {
	str = strcase.ToSnake(str)
	str = strings.ReplaceAll(str, "pitch_class", "pitchclass")
	return str
}
