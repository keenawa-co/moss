package utils

import (
	"go/ast"
	"go/token"
)

func CalcNodeLOC(fset *token.FileSet, node ast.Node) int {
	return fset.Position(node.End()).Line - fset.Position(node.Pos()).Line + 1
}
