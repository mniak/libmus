package models

import "go/ast"

type Function struct {
	Struct     string
	Name       string
	ReturnType string
}

func ParseFunction(fd *ast.FuncDecl) Function {
	fn := Function{
		Name: fd.Name.Name,
	}
	if fd.Recv != nil {
		fn.Struct = mustFind[*ast.TypeSpec](fd.Recv.List[0].Type).Name.Name
	}
	return fn
}
