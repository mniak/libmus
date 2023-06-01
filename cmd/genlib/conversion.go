package main

import (
	"fmt"
	"go/ast"
	"go/token"

	"github.com/mniak/libmus/cmd/models"
)

type (
	ConversionFunc func(e ast.Expr) ast.Expr
	Conversion     struct {
		IsTrivial bool
		Convert   ConversionFunc
	}
)

type TypeInfo struct {
	CType string
	CToGo Conversion
	GoToC Conversion
}

func TrivialConversion() Conversion {
	return Conversion{
		IsTrivial: true,
		Convert:   func(e ast.Expr) ast.Expr { return e },
	}
}

func NewConversion(fn ConversionFunc) Conversion {
	return Conversion{
		IsTrivial: false,
		Convert:   fn,
	}
}

func (c Conversion) Wrap(outer Conversion) Conversion {
	if c.IsTrivial && outer.IsTrivial {
		return TrivialConversion()
	}
	return NewConversion(func(e ast.Expr) ast.Expr {
		return outer.Convert(c.Convert(e))
	})
}

func TypeAssertion(typeIndent string) Conversion {
	return NewConversion(func(e ast.Expr) ast.Expr {
		return &ast.TypeAssertExpr{
			X:    e,
			Type: ast.NewIdent(typeIndent),
		}
	})
}

func Call(funcIdent string) Conversion {
	return NewConversion(func(e ast.Expr) ast.Expr {
		return &ast.CallExpr{
			Fun:  ast.NewIdent(funcIdent),
			Args: []ast.Expr{e},
		}
	})
}

func Select(selectionIdent string) Conversion {
	return NewConversion(func(e ast.Expr) ast.Expr {
		return &ast.SelectorExpr{
			X:   e,
			Sel: ast.NewIdent(selectionIdent),
		}
	})
}

func StructToHandle(cTypeIdent string) Conversion {
	return Call("cgo.NewHandle").Wrap(Call(cTypeIdent))
}

func HandleToStruct(goTypeIdent string) Conversion {
	return Call("cgo.Handle").
		Wrap(Select("Value()")).
		Wrap(TypeAssertion(goTypeIdent))
}

func ToPointer() Conversion {
	return NewConversion(func(e ast.Expr) ast.Expr {
		return &ast.UnaryExpr{
			X:  e,
			Op: token.AND,
		}
	})
}

func getTypeInfo(t models.Type) *TypeInfo {
	if t == nil {
		return nil
	}
	if t.IsPointer() && t.AsPointer().IsStruct() {
		return &TypeInfo{
			CType: fmt.Sprintf("C.%s", t.Name()),
			CToGo: HandleToStruct(fmt.Sprintf("*%s.%s", libName, t.Name())),
			GoToC: StructToHandle(fmt.Sprintf("C.%s", t.Name())),
		}
	}
	if t.IsStruct() {
		return &TypeInfo{
			CType: fmt.Sprintf("C.%s", t.Name()),
			CToGo: HandleToStruct(fmt.Sprintf("%s.%s", libName, t.Name())),
			GoToC: ToPointer().
				Wrap(Call("cgo.NewHandle")).
				Wrap(Call(fmt.Sprintf("C.%s", t.Name()))),
		}
	}
	if t.Name() == "string" {
		return &TypeInfo{
			CType: "*C.char",
			CToGo: Call("C.GoString"),
			GoToC: Call("C.CString"),
		}
	}
	return &TypeInfo{
		CType: t.Name(),
		CToGo: TrivialConversion(),
		GoToC: TrivialConversion(),
	}
}
