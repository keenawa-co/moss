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
		return nil, fmt.Errorf("some error from pickStructType : %s not a *ast.TypeSpec", reflect.TypeOf(node).String()) // TODO: add normal error return message
	}

	structType, ok := typeSpec.Type.(*ast.StructType)
	if !ok {
		return nil, fmt.Errorf("some error from pickStructType : %s not a *ast.StructType", reflect.TypeOf(node).String()) // TODO: add normal error return message
	}

	typeObj := new(obj.TypeObj)

	if typeSpec.TypeParams != nil && len(typeSpec.TypeParams.List) > 0 {
		var err error
		typeObj.TypeParams, err = obj.ProcessFieldList(state.File, typeSpec.TypeParams, typeObj.ImportAdder)
		if err != nil {
			return nil, err
		}
	}

	structTypeObj, err := obj.NewStructTypeObj(state.File, structType)
	if err != nil {
		return nil, errors.New("some error from pickStructType 3") // TODO: add normal error return message
	}

	typeObj.Pos = state.File.FileSet.Position(typeSpec.Pos()).Line
	typeObj.End = state.File.FileSet.Position(typeSpec.End()).Line
	typeObj.Name = obj.NewIdentObj(typeSpec.Name)
	typeObj.Type = structTypeObj
	typeObj.TypeKind = obj.Typ

	return typeObj, nil
}

func pickFuncType(state *state.State, node ast.Node) (obj.Object, error) {
	ts, ok := node.(*ast.TypeSpec)
	if !ok {
		return nil, fmt.Errorf("some error from pickFuncType : %s not a *ast.TypeSpec", reflect.TypeOf(node).String()) // TODO: add normal error return message
	}

	funcType, ok := ts.Type.(*ast.FuncType)
	if !ok {
		return nil, fmt.Errorf("some error from pickFuncType : %s not a *ast.StructType", reflect.TypeOf(node).String()) // TODO: add normal error return message
	}

	typeObj := new(obj.TypeObj)

	if ts.TypeParams != nil && len(ts.TypeParams.List) > 0 {
		var err error
		typeObj.TypeParams, err = obj.ProcessFieldList(state.File, ts.TypeParams, typeObj.ImportAdder)
		if err != nil {
			return nil, err
		}
	}

	funcTypeObj, err := obj.NewFuncTypeObj(state.File, funcType)
	if err != nil {
		return nil, errors.New("some error from pickFuncType 3") // TODO: add normal error return message
	}

	typeObj.Pos = state.File.FileSet.Position(ts.Pos()).Line
	typeObj.End = state.File.FileSet.Position(ts.End()).Line
	typeObj.Name = obj.NewIdentObj(ts.Name)
	typeObj.Type = funcTypeObj
	typeObj.TypeKind = obj.Typ

	return typeObj, nil
}
