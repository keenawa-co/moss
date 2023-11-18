package compass

import (
	"errors"
	"go/ast"
	"strings"

	"github.com/4rchr4y/go-compass/obj"
)

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
