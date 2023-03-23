package models

import "go/ast"

type parsedType struct {
	astNode any
}

type Type interface {
	Name() string
	IsStruct() bool
	PointedType() Type
}

func ParseType(n any) Type {
	return &parsedType{
		astNode: n,
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

func (t *parsedType) IsStruct() bool {
	return isStruct(t.astNode)
}

func (t *parsedType) Name() string {
	switch node := t.astNode.(type) {
	case *ast.Ident:
		return node.Name
	case *ast.TypeSpec:
		return node.Name.Name
	case *ast.StarExpr:
		return ParseType(node.X).Name()
	default:
		return ""
	}
}

func (t *parsedType) PointedType() Type {
	if t.astNode == nil {
		return nil
	}
	pointer, is := t.astNode.(*ast.StarExpr)
	if !is {
		return nil
	}
	return ParseType(pointer.X)
}
