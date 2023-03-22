package models

import "go/ast"

type Type struct {
	astNode any
	Name    string
}

func ParseType(n any) *Type {
	switch node := n.(type) {
	case *ast.Ident:
		return &Type{
			astNode: node,
			Name:    node.Name,
		}
	case *ast.TypeSpec:
		return &Type{
			astNode: node,
			Name:    node.Name.Name,
		}
	case *ast.StarExpr:
		return &Type{
			astNode: node,
			Name:    ParseType(node.X).Name,
		}
	default:
		return nil
	}
}

func isStruct(n any) bool {
	if n == nil {
		return false
	}
	switch v := n.(type) {
	case *ast.Ident:
		if v.Obj != nil {
			return isStruct(v.Obj)
		}
		return false
	case *ast.Object:
		if v.Decl != nil {
			return isStruct(v.Decl)
		}
		return false
	case *ast.TypeSpec:
		return true
	default:
		return false
	}
}

func (t *Type) IsStruct() bool {
	return isStruct(t.astNode)
}

func (t *Type) PointedType() *Type {
	if t.astNode == nil {
		return nil
	}
	pointer, is := t.astNode.(*ast.StarExpr)
	if !is {
		return nil
	}
	return ParseType(pointer.X)
}
