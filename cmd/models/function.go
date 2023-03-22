package models

import (
	"go/ast"
	"strings"

	"github.com/iancoleman/strcase"
)

type Function struct {
	Struct      *Type
	Name        string
	Return      *Type
	Parameters  []Parameter
	Constructor bool
}

type Parameter struct {
	Type Type
	Name string
}

func ParseFunction(fd *ast.FuncDecl) Function {
	fn := Function{
		Name: fd.Name.Name,
	}
	if fd.Recv != nil {
		fn.Struct = ParseType(fd.Recv.List[0].Type)
	}

	for _, param := range fd.Type.Params.List {
		typ := ParseType(param.Type)
		var name string
		if len(param.Names) > 0 {
			name = param.Names[0].Name
		} else {
			name = strcase.ToSnake(typ.Name)
		}
		if typ == nil {
			panic("could not parse return type of function")
		}
		fn.Parameters = append(fn.Parameters, Parameter{
			Type: *typ,
			Name: name,
		})
	}
	if fd.Type.Results == nil {
		return fn
	}
	switch len(fd.Type.Results.List) {
	case 0:
	case 1:
		fn.Return = ParseType(fd.Type.Results.List[0].Type)
		if fn.Return.IsStruct() && strings.Contains(fn.Name, fn.Return.Name) {
			fn.Struct = fn.Return
			fn.Constructor = true
		}
	default:
		panic("functions returning more than one value are not supported")
	}
	return fn
}
