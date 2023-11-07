package compass

import (
	"context"
	"fmt"
	"go/ast"
	"sync"

	"github.com/4rchr4y/go-compass/obj"
)

type Visitor interface {
	// Custom implementation of a standard ast.Walk function.
	// Was implemented because the standard ast.Walk function does not have a context inside.
	VisitWithContext(ctx context.Context, node ast.Node) (w Visitor)
}

type visitor struct {
	noCopy noCopy

	// Data structure for the analyzed file
	fileObj *obj.FileObj

	// Created map of analyzers for a specific file
	analyzerGroup AnalyzerGroup

	once sync.Once
}

type visitorConfig struct {
	file   *obj.FileObj
	alzMap AnalyzerGroup
}

func NewVisitor(cfg visitorConfig) *visitor {
	v := new(visitor)
	v.once.Do(func() {
		v.fileObj = cfg.file
		v.analyzerGroup = cfg.alzMap

	})

	return v
}

func (v *visitor) VisitWithContext(ctx context.Context, node ast.Node) Visitor {
	if node == nil {
		return v
	}

	analyzer, ok := v.analyzerGroup.Search(node)
	if !ok {
		return v
	}

	object, err := analyzer.Analyze(ctx, node)
	if err != nil {
		fmt.Println(err) // TODO: decide later how to handle the error
		return v
	}

	if err := v.fileObj.Save(object); err != nil {
		fmt.Println(err) // TODO: decide later how to handle the error
	}

	return v
}
