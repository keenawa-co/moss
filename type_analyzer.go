package compass

import (
	"errors"
	"fmt"
	"go/ast"
	"reflect"

	"github.com/4rchr4y/go-compass/obj"
	"github.com/4rchr4y/go-compass/state"
)

func NewFuncTypeAnalyzer() Analyzer[ast.Node, obj.Object] {
	return NewAnalyzer[ast.Node, obj.Object](analyzeFuncType)
}

func NewStructTypeAnalyzer() Analyzer[ast.Node, obj.Object] {
	return NewAnalyzer[ast.Node, obj.Object](analyzeStructType)
}

func analyzeStructType(state *state.State, node ast.Node) (obj.Object, error) {
	typeSpec, ok := node.(*ast.TypeSpec)
	if !ok {
		return nil, fmt.Errorf("some error from analyzeStructNode : %s", reflect.TypeOf(node).String()) // TODO: add normal error return message
	}

	structObj, usedPackages, err := obj.NewStructObj(state.File.FileSet, typeSpec, &typeSpec.Name.Name)
	if err != nil {
		return nil, errors.New("some error from analyzeStructNode 3") // TODO: add normal error return message
	}

	for _, pkg := range usedPackages {
		if index, exists := state.File.Entities.Imports.InternalImportsMeta[pkg.Alias]; exists {
			structObj.AddDependency(index, pkg.Element)
		}
	}

	typeObject, err := obj.NewTypeObj(state.File, typeSpec)
	if err != nil {
		return nil, errors.New("some error from analyzeStructNode 4") // TODO: add normal error return message
	}

	typeObject.EmbedObject(structObj)

	return typeObject, nil
}

func analyzeFuncType(state *state.State, node ast.Node) (obj.Object, error) {
	typeSpec, ok := node.(*ast.TypeSpec)
	if !ok {
		return nil, fmt.Errorf("some error from analyzeStructNode : %s", reflect.TypeOf(node).String()) // TODO: add normal error return message
	}

	funcTypeObj, err := obj.NewFuncTypeObj(state.File.FileSet, node)
	if err != nil {
		return nil, fmt.Errorf("some error from analyzeStructNode %w", err) // TODO: add normal error return message
	}

	typeObject, err := obj.NewTypeObj(state.File, typeSpec)
	if err != nil {
		return nil, errors.New("some error from analyzeStructNode 4") // TODO: add normal error return message
	}

	typeObject.EmbedObject(funcTypeObj)

	return typeObject, nil
}
