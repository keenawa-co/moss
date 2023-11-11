package compass

import (
	"go/ast"
	"reflect"

	"github.com/4rchr4y/go-compass/obj"
	"github.com/4rchr4y/go-compass/picker"
)

type (
	// PickerFactoryGroup is a map that associates keys of a specified type (Key) with PickerFactory functions.
	// It is used to store and retrieve PickerFactory functions that can create Pickers for various types.
	PickerFactoryGroup map[reflect.Type]picker.PickerFactory[obj.Object]

	// PickerGroup is a map that associates keys of a specified type (Key) with Picker instances.
	// It is used to store and retrieve Picker implementations for various types.
	PickerGroup map[reflect.Type]picker.Picker[obj.Object]
)

func (pickFacGroup PickerFactoryGroup) Make() PickerGroup {
	result := make(PickerGroup, len(pickFacGroup))

	for analyzedType, analyzerFactory := range pickFacGroup {
		result[analyzedType] = analyzerFactory()
	}

	return result
}

func (pickGroup PickerGroup) Search(node ast.Node) (picker.Picker[obj.Object], bool) {
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
