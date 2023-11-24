package goray

import (
	"go/ast"
	"reflect"
)

type (
	// PickerFactoryGroup is a map that associates keys of a specified type (Key) with PickerFactory functions.
	// It is used to store and retrieve PickerFactory functions that can create Pickers for various types.
	PickerFactoryGroup map[reflect.Type]PickerFactory

	// PickerGroup is a map that associates keys of a specified type (Key) with Picker instances.
	// It is used to store and retrieve Picker implementations for various types.
	PickerGroup map[reflect.Type]Picker
)

func (pickFacGroup PickerFactoryGroup) Make() PickerGroup {
	result := make(PickerGroup, len(pickFacGroup))

	for analyzedType, analyzerFactory := range pickFacGroup {
		result[analyzedType] = analyzerFactory()
	}

	return result
}

func (pickGroup PickerGroup) Search(node ast.Node) (Picker, bool) {
	switch n := node.(type) {
	case *ast.ImportSpec:
		return pickGroup[reflect.TypeOf(new(ast.ImportSpec))], true

	case *ast.FuncDecl:
		return pickGroup[reflect.TypeOf(new(ast.FuncDecl))], true

	case *ast.TypeSpec:
		switch n.Type.(type) {
		case *ast.StructType:
			return pickGroup[reflect.TypeOf(new(ast.StructType))], true

		case *ast.FuncType:
			return pickGroup[reflect.TypeOf(new(ast.FuncType))], true
		}
	}

	return nil, false
}
