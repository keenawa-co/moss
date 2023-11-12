package obj

import (
	"bytes"
	"fmt"
	"go/ast"
	"go/format"
	"go/token"
	"reflect"
)

type IdentObj struct {
	Name string
}

func (idObj *IdentObj) IsExported() bool {
	return token.IsExported(idObj.Name)
}

func NewIdentObj(id *ast.Ident) *IdentObj {
	return &IdentObj{
		Name: id.Name,
	}
}

func mapIdentAstArray(array []*ast.Ident) []*IdentObj {
	result := make([]*IdentObj, 0, len(array))
	for _, name := range array {
		result = append(result, NewIdentObj(name))
	}

	return result
}

type FieldObj struct {
	Names []*IdentObj
	Type  any
}

type BlockStmtObj struct {
	Lbrace       token.Pos // position of "{"
	Dependencies map[string]int
	Rbrace       token.Pos // position of "}", if any (may be absent due to syntax error)
}

func (o *BlockStmtObj) ImportAdder(importIndex int, element string) {
	if o.Dependencies == nil {
		o.Dependencies = make(map[string]int)
	}

	o.Dependencies[element] = importIndex
}

func CalcNodeLOC(fset *token.FileSet, node ast.Node) int {
	return fset.Position(node.End()).Line - fset.Position(node.Pos()).Line + 1
}

func determineExprType(fobj *FileObj, expr ast.Expr, adder func(index int, name string)) (any, error) {
	switch e := expr.(type) {
	case *ast.StructType:
		return NewStructObj(fobj, e, nil)

	case *ast.SelectorExpr:
		ident, ok := e.X.(*ast.Ident)
		if !ok {
			return e.Sel.Name, nil
		}

		if index, exists := fobj.Entities.Imports.InternalImportsMeta[ident.Name]; exists {
			adder(index, e.Sel.Name)
		}

		return e.Sel.Name, nil

	case *ast.StarExpr:
		// This branch, in my understanding, is used only to determine the receiver of methods.
		// Getting into this branch means that receiver's type is a pointer.

		// attempt to get the name of the struct
		if ident, ok := e.X.(*ast.Ident); ok {
			return ident.Name, nil
		}

		return nil, fmt.Errorf("failed to get expr %s type", reflect.TypeOf(e.X))

	case *ast.Ident:
		// This branch, in my understanding, is used only to determine the receiver of methods.
		// Getting into this branch means that receiver's type is not a pointer, it's a regular struct.
		// So return the name of the struct.
		return e.Name, nil

	default:
		var buf bytes.Buffer
		if err := format.Node(&buf, fobj.FileSet, e); err != nil {
			return nil, fmt.Errorf("failed to format expr: %w", err)
		}

		return buf.String(), nil
	}
}

func processField(fobj *FileObj, field *ast.Field, adder func(index int, name string)) (*FieldObj, error) {
	typ, err := determineExprType(fobj, field.Type, adder)
	if err != nil {
		return nil, err
	}

	return &FieldObj{
		Names: mapIdentAstArray(field.Names),
		Type:  typ,
	}, nil
}

func processFieldList(fobj *FileObj, fieldList []*ast.Field, adder func(index int, name string)) ([]*FieldObj, error) {
	result := make([]*FieldObj, 0, len(fieldList))

	for _, field := range fieldList {
		fieldObj, err := processField(fobj, field, adder)
		if err != nil {
			return nil, err
		}

		result = append(result, fieldObj)
	}

	return result, nil
}
