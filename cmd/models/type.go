package models

import "go/ast"

type Type struct {
	astNode any
	Name    string
}

func ParseType(n any) Type {
	switch node := n.(type) {
	case *ast.Ident:
		return Type{
			astNode: node,
			Name:    node.Name,
		}
	case *ast.StarExpr:
		return ParseType(node.X)
	case *ast.TypeSpec:
		return Type{
			astNode: node,
			Name:    node.Name.Name,
		}

	default:
		panic("could not parse type")
	}
}

func (t *Type) IsStruct() bool {
	if t.astNode == nil {
		return false
	}
	_, is := find[*ast.TypeSpec](t.astNode)
	if is {
		return true
	}
	return false
}
