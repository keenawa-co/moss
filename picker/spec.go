package picker

import (
	"errors"
	"go/ast"
	"strings"

	"github.com/4rchr4y/go-compass/obj"
	"github.com/4rchr4y/go-compass/state"
)

func NewImportSpecPicker() Picker[obj.Object] {
	return NewPicker[obj.Object](pickImportSpec)
}

func pickImportSpec(state *state.State, node ast.Node) (obj.Object, error) {
	importSpec, _ := node.(*ast.ImportSpec)

	if importSpec.Path == nil && importSpec.Path.Value == "" {
		return nil, errors.New("some error from analyzeImportNode 1") // TODO: add normal error return message
	}

	// by default, from ast we get an import string that is wrapped in quotes
	importSpec.Path.Value = strings.Trim(importSpec.Path.Value, `"`)

	if !strings.HasPrefix(importSpec.Path.Value, state.File.Metadata.Module) {
		return obj.NewImportObj(importSpec, obj.ImportTypeExternal), nil
	}

	if importSpec.Name != nil && importSpec.Name.Name == "_" {
		return obj.NewImportObj(importSpec, obj.ImportTypeSideEffect), nil
	}

	return obj.NewImportObj(importSpec, obj.ImportTypeInternal), nil
}
