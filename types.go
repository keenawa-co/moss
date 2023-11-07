package compass

import (
	"go/ast"
	"reflect"

	"github.com/4rchr4y/go-compass/analyzer"
	"github.com/4rchr4y/go-compass/obj"
)

type (
	AnalyzerFactoryGroup analyzer.AnalyzerFactoryMap[reflect.Type, ast.Node, obj.Object]
	AnalyzerGroup        analyzer.AnalyzerMap[reflect.Type, ast.Node, obj.Object]
)

func (afg AnalyzerFactoryGroup) Make(f *obj.FileObj) AnalyzerGroup {
	result := make(AnalyzerGroup, len(afg))
	for k, v := range afg {
		result[k] = v(f)
	}

	return result
}

func (ag AnalyzerGroup) Search(node ast.Node) (analyzer.Analyzer[ast.Node, obj.Object], bool) {
	switch n := node.(type) {
	case *ast.ImportSpec:
		return ag[reflect.TypeOf(new(ast.ImportSpec))], true

	case *ast.FuncDecl:
		return ag[reflect.TypeOf(new(ast.FuncDecl))], true

	case *ast.TypeSpec:
		switch n.Type.(type) {
		case *ast.StructType:
			return ag[reflect.TypeOf(new(ast.StructType))], true

		case *ast.FuncType:
			return ag[reflect.TypeOf(new(ast.FuncType))], true
		}
	}

	return nil, false
}
