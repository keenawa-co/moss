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
		importObj := obj.NewImportObj(importSpec, obj.ImportTypeSideEffect)
		importObj.Path = path

		return importObj, nil
	}

	if !hasModPrefix {
		importObj := obj.NewImportObj(importSpec, obj.ImportTypeExternal)
		importObj.Path = path

		return importObj, nil
	}

	importObj := obj.NewImportObj(importSpec, obj.ImportTypeInternal)
	importObj.Path = path

	return importObj, nil
}
