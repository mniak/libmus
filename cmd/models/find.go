package models

import (
	"go/ast"
)

func mustFind[T any](a any) (val T) {
	v, ok := find[T](a)
	if !ok {
		panic("find failed")
	}
	return v
}

func find[T any](a any) (val T, found bool) {
	if a == nil {
		return
	}

	if conv, ok := a.(T); ok {
		return conv, true
	}

	switch node := a.(type) {
	case *ast.StarExpr:
		return find[T](node.X)
	case *ast.Ident:
		if node.Obj == nil {
			return val, false
		}
		return find[T](node.Obj.Decl)
	}
	return
}
