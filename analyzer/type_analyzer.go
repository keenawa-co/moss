package analyzer

import (
	"context"
	"errors"
	"fmt"
	"go/ast"
	"reflect"

	"github.com/4rchr4y/go-compass/obj"
)

func NewFuncTypeAnalyzer(file *obj.FileObj) Analyzer[ast.Node, obj.Object] {
	return NewAnalyzer[ast.Node, obj.Object](
		file,
		analyzeFuncType,
	)
}

func NewStructTypeAnalyzer(file *obj.FileObj) Analyzer[ast.Node, obj.Object] {
	return NewAnalyzer[ast.Node, obj.Object](
		file,
		analyzeStructType,
	)
}

func analyzeStructType(ctx context.Context, f *obj.FileObj, node ast.Node) (obj.Object, error) {
	typeSpec, ok := node.(*ast.TypeSpec)
	if !ok {
		return nil, fmt.Errorf("some error from analyzeStructNode : %s", reflect.TypeOf(node).String()) // TODO: add normal error return message
	}

	structObj, usedPackages, err := obj.NewStructObj(f.FileSet, typeSpec, &typeSpec.Name.Name)
	if err != nil {
		return nil, errors.New("some error from analyzeStructNode 3") // TODO: add normal error return message
	}

	for _, pkg := range usedPackages {
		if index, exists := f.Entities.Imports.InternalImportsMeta[pkg.Alias]; exists {
			structObj.AddDependency(index, pkg.Element)
		}
	}

	typeObject, err := obj.NewTypeObj(f, typeSpec)
	if err != nil {
		return nil, errors.New("some error from analyzeStructNode 4") // TODO: add normal error return message
	}

	typeObject.EmbedObject(structObj)

	return typeObject, nil
}

func analyzeFuncType(ctx context.Context, f *obj.FileObj, node ast.Node) (obj.Object, error) {
	typeSpec, ok := node.(*ast.TypeSpec)
	if !ok {
		return nil, fmt.Errorf("some error from analyzeStructNode : %s", reflect.TypeOf(node).String()) // TODO: add normal error return message
	}

	funcTypeObj, err := obj.NewFuncTypeObj(f.FileSet, node)
	if err != nil {
		return nil, fmt.Errorf("some error from analyzeStructNode %w", err) // TODO: add normal error return message
	}

	typeObject, err := obj.NewTypeObj(f, typeSpec)
	if err != nil {
		return nil, errors.New("some error from analyzeStructNode 4") // TODO: add normal error return message
	}

	typeObject.EmbedObject(funcTypeObj)

	return typeObject, nil
}
