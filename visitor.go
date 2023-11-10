package compass

import (
	"fmt"
	"go/ast"
	"sync"

	"github.com/4rchr4y/go-compass/core"
	"github.com/4rchr4y/go-compass/state"
)

type Visitor interface {
	// Custom implementation of a standard ast.Walk function.
	Visit(ctx *state.State, node ast.Node) (w Visitor)
}

type visitor struct {
	noCopy core.NoCopy

	// Created map of analyzers for a specific file
	analyzerGroup AnalyzerGroup

	once sync.Once
}

func NewVisitor(group AnalyzerFactoryGroup) *visitor {
	v := new(visitor)
	v.once.Do(func() {
		v.analyzerGroup = group.Make()

	})

	return v
}

func (v *visitor) Visit(state *state.State, node ast.Node) Visitor {
	if node == nil {
		return v
	}

	analyzer, ok := v.analyzerGroup.Search(node)
	if !ok {
		return v
	}

	object, err := analyzer.Analyze(state, node)
	if err != nil {
		fmt.Println(err) // TODO: decide later how to handle the error
		return v
	}

	if err := state.File.Save(object); err != nil {
		fmt.Println(err) // TODO: decide later how to handle the error
	}

	return v
}
