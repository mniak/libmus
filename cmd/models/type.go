package models

import (
	"go/ast"
)

type parsedType struct {
	astNode any
}

type Type interface {
	Name() string
	IsStruct() bool
	IsPointer() bool
	AsPointed() Type
	AsPointer() Type
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
	case *ast.StarExpr:
		return isStruct(v.X)
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

func (t *parsedType) IsPointer() bool {
	_, is := t.astNode.(*ast.StarExpr)
	return is
}

func (t *parsedType) AsPointed() Type {
	starType, is := t.astNode.(*ast.StarExpr)
	if is {
		return ParseType(starType.X)
	}
	return t
}

func (t *parsedType) AsPointer() Type {
	if t.astNode == nil {
		return nil
	}
	_, isPointer := t.astNode.(*ast.StarExpr)
	if isPointer {
		return t
	}

	return &pointerType{
		inner: t,
	}
}

type pointerType struct {
	inner Type
}

func (t *pointerType) IsStruct() bool {
	return t.inner.IsStruct()
}

func (t *pointerType) Name() string {
	return "*" + t.Name()
}

func (t *pointerType) IsPointer() bool {
	return true
}

func (t *pointerType) AsPointed() Type {
	return t.inner
}

func (t *pointerType) AsPointer() Type {
	return t
}
