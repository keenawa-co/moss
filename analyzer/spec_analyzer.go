package analyzer

import (
	"context"
	"errors"
	"go/ast"
	"strings"

	"github.com/4rchr4y/go-compass/obj"
)

func NewImportSpecAnalyzer(file *obj.FileObj) Analyzer[ast.Node, obj.Object] {
	return NewAnalyzer[ast.Node, obj.Object](
		file,
		analyzeImportSpec,
	)
}

func analyzeImportSpec(ctx context.Context, f *obj.FileObj, node ast.Node) (obj.Object, error) {
	importSpec, _ := node.(*ast.ImportSpec)

	if importSpec.Path == nil && importSpec.Path.Value == "" {
		return nil, errors.New("some error from analyzeImportNode 1") // TODO: add normal error return message
	}

	path := strings.Trim(importSpec.Path.Value, `"`)
	if !strings.HasPrefix(path, f.Metadata.Module) {
		return obj.NewImportObj(importSpec, obj.ImportTypeExternal), nil
	}

	if importSpec.Name != nil && importSpec.Name.Name == "_" {
		return obj.NewImportObj(importSpec, obj.ImportTypeSideEffect), nil
	}

	return obj.NewImportObj(importSpec, obj.ImportTypeInternal), nil
}
