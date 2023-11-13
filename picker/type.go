package picker

import (
	"errors"
	"fmt"
	"go/ast"
	"reflect"

	"github.com/4rchr4y/go-compass/obj"
	"github.com/4rchr4y/go-compass/state"
)

func NewFuncTypePicker() Picker[obj.Object] {
	return NewPicker[obj.Object](pickFuncType)
}

func NewStructTypePicker() Picker[obj.Object] {
	return NewPicker[obj.Object](pickStructType)
}

func pickStructType(state *state.State, node ast.Node) (obj.Object, error) {
	typeSpec, ok := node.(*ast.TypeSpec)
	if !ok {
		return nil, fmt.Errorf("some error from analyzeStructNode : %s", reflect.TypeOf(node).String()) // TODO: add normal error return message
	}

	// structObj, err := obj.NewStructObj(state.File, typeSpec, &typeSpec.Name.Name)
	// if err != nil {
	// 	return nil, errors.New("some error from analyzeStructNode 3") // TODO: add normal error return message
	// }

	typeObject, err := obj.NewTypeObj(state.File, typeSpec)
	if err != nil {
		return nil, errors.New("some error from analyzeStructNode 4") // TODO: add normal error return message
	}

	// typeObject.EmbedObject(structObj)

	return typeObject, nil
}

func pickFuncType(state *state.State, node ast.Node) (obj.Object, error) {
	ts, ok := node.(*ast.TypeSpec)
	if !ok {
		return nil, fmt.Errorf("some error from analyzeStructNode : %s", reflect.TypeOf(node).String()) // TODO: add normal error return message
	}

	// _, ok = ts.Type.(*ast.FuncType)
	// if !ok {
	// 	return nil, fmt.Errorf("node is not a FuncType: %s", reflect.TypeOf(node))
	// }

	// funcTypeObj, err := obj.NewFuncTypeObj(state.File, funcType)
	// if err != nil {
	// 	return nil, fmt.Errorf("some error from analyzeStructNode %w", err) // TODO: add normal error return message
	// }

	typeObject, err := obj.NewTypeObj(state.File, ts)
	if err != nil {
		return nil, errors.New("some error from analyzeStructNode 4") // TODO: add normal error return message
	}

	// typeObject.EmbedObject(funcTypeObj)

	return typeObject, nil
}
