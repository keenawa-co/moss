package obj

import (
	"bytes"
	"fmt"
	"go/ast"
	"go/format"
	"go/token"
)

type DependencyObj struct {
	ImportIndex int
	Usage       int
}

type astExpr interface {
	Pos() token.Pos
	End() token.Pos
}

func CalcEntityLOC(fset *token.FileSet, expr astExpr) int {
	return fset.Position(expr.End()).Line - fset.Position(expr.Pos()).Line + 1
}

type exprTypeMetaData struct {
	Type           string
	UsedPackages   []UsedPackage
	EmbeddedStruct *StructTypeObj
}

func ExtractExprAsType(fset *token.FileSet, expr ast.Node) (*exprTypeMetaData, error) {
	switch ft := expr.(type) {
	case *ast.StructType:
		embedded, usedPackages, err := NewStructObj(fset, ft, nil)
		if err != nil {
			return nil, err
		}

		return &exprTypeMetaData{
			Type:           "struct",
			UsedPackages:   usedPackages,
			EmbeddedStruct: embedded,
		}, nil

	case *ast.SelectorExpr:
		if ident, ok := ft.X.(*ast.Ident); ok {
			return &exprTypeMetaData{
				Type:         ft.Sel.Name,
				UsedPackages: []UsedPackage{{Alias: ident.Name, Element: ft.Sel.Name}},
			}, nil
		}

		return &exprTypeMetaData{
			Type: ft.Sel.Name,
		}, nil

	default:
		var buf bytes.Buffer
		if err := format.Node(&buf, fset, expr); err != nil {
			return nil, fmt.Errorf("failed to format node: %w", err)
		}

		return &exprTypeMetaData{
			Type: buf.String(),
		}, nil
	}
}
