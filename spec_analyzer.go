package compass

import (
	"errors"
	"go/ast"
	"strings"

	"github.com/4rchr4y/go-compass/obj"
	"github.com/4rchr4y/go-compass/state"
)

func NewImportSpecAnalyzer() Analyzer[ast.Node, obj.Object] {
	return NewAnalyzer[ast.Node, obj.Object](analyzeImportSpec)
}

func analyzeImportSpec(state *state.State, node ast.Node) (obj.Object, error) {
	importSpec, _ := node.(*ast.ImportSpec)

	if importSpec.Path == nil && importSpec.Path.Value == "" {
		return nil, errors.New("some error from analyzeImportNode 1") // TODO: add normal error return message
	}

	path := strings.Trim(importSpec.Path.Value, `"`)
	if !strings.HasPrefix(path, state.File.Metadata.Module) {
		return obj.NewImportObj(importSpec, obj.ImportTypeExternal), nil
	}

	if importSpec.Name != nil && importSpec.Name.Name == "_" {
		return obj.NewImportObj(importSpec, obj.ImportTypeSideEffect), nil
	}

	return obj.NewImportObj(importSpec, obj.ImportTypeInternal), nil
}
