package compass

import (
	"errors"
	"fmt"
	"go/ast"
	"reflect"
	"strings"

	"github.com/4rchr4y/go-compass/obj"
)

type Picker interface {
	Pick(state *State, node ast.Node) (obj.Object, error)
}

type (
	PickerFactory func() Picker
	PickFunc      func(s *State, node ast.Node) (obj.Object, error)
)

func NewPicker(pick PickFunc) Picker {
	return &picker{
		pick: pick,
	}
}

type picker struct {
	pick PickFunc
}

func (a *picker) Pick(s *State, n ast.Node) (obj.Object, error) {
	return a.pick(s, n)
}

// ----------------------------------------------------------------------------
// Types pickers
//
// The creation of pickers, which allow for the extraction and analysis of
// specific data types (structures and functional types) from an AST
// (Abstract Syntax Tree), representing the structure of the source code.

func NewStructTypePicker() Picker {
	return NewPicker(pickStructType)
}

func pickStructType(state *State, node ast.Node) (obj.Object, error) {
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

	typeObj.Type = structTypeObj
	typeObj.Pos = state.File.FileSet.Position(typeSpec.Pos()).Line
	typeObj.End = state.File.FileSet.Position(typeSpec.End()).Line
	typeObj.Name = &obj.IdentObj{
		Name: typeSpec.Name.Name,
		Kind: obj.Typ,
	}

	return typeObj, nil
}

func NewFuncTypePicker() Picker {
	return NewPicker(pickFuncType)
}

func pickFuncType(state *State, node ast.Node) (obj.Object, error) {
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
	typeObj.Type = funcTypeObj
	typeObj.Name = &obj.IdentObj{
		Name: ts.Name.Name,
		Kind: obj.Typ,
	}

	return typeObj, nil
}

// ----------------------------------------------------------------------------
// Declaration pickers
//
// Identifying and extracting declarations from an AST
// (Abstract Syntax Tree) of Go source code.

func NewFuncDeclPicker() Picker {
	return NewPicker(pickFuncDecl)
}

func pickFuncDecl(state *State, node ast.Node) (obj.Object, error) {
	decl, _ := node.(*ast.FuncDecl)

	funcDeclObj, err := obj.NewFuncDeclObj(state.File, decl)
	if err != nil {

		return nil, fmt.Errorf("some error from pickFuncDecl 1 %w", err) // TODO: add normal error return message
	}

	return &obj.DeclObj{
		Pos: state.File.FileSet.Position(decl.Pos()).Line,
		End: state.File.FileSet.Position(decl.End()).Line,
		Name: &obj.IdentObj{
			Name: decl.Name.Name,
			Kind: obj.Fun,
		},
		Type: funcDeclObj,
	}, nil
}

// ----------------------------------------------------------------------------
// Specs pickers
//
// Identifying and extracting specs from an AST
// (Abstract Syntax Tree) of Go source code.

func NewImportSpecPicker() Picker {
	return NewPicker(pickImportSpec)
}

func pickImportSpec(state *State, node ast.Node) (obj.Object, error) {
	importSpec, ok := node.(*ast.ImportSpec)
	if !ok {
		return nil, errors.New("node is not an ImportSpec")
	}

	if importSpec.Path == nil {
		return nil, errors.New("import spec path is nil")
	}

	// from ast get an import string that is wrapped in quotes
	path := strings.Trim(importSpec.Path.Value, `"`)
	hasModPrefix := strings.HasPrefix(path, state.Modfile.Module.Mod.Path)
	if hasModPrefix {
		path = path[len(state.Modfile.Module.Mod.Path):]
	}

	if importSpec.Name != nil && importSpec.Name.Name == "_" {
		importObj := obj.NewImportObj(importSpec, obj.SideEffect)
		importObj.Path = path

		return importObj, nil
	}

	if !hasModPrefix {
		importObj := obj.NewImportObj(importSpec, obj.External)
		importObj.Path = path

		return importObj, nil
	}

	importObj := obj.NewImportObj(importSpec, obj.Internal)
	importObj.Path = path

	return importObj, nil
}
