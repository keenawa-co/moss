package analyzer

import (
	"context"

	"github.com/4rchr4y/go-compass/obj"
)

type Analyzer[Input, Output any] interface {
	Analyze(ctx context.Context, i Input) (Output, error)
}

type (
	// AnalyzerFactoryMap is a map that associates keys of a specified type (Key) with AnalyzerFactory functions.
	// It is used to store and retrieve AnalyzerFactory functions that can create Analyzers for various types.
	AnalyzerFactoryMap[Key comparable, Input, Output any] map[Key]AnalyzerFactory[Input, Output]

	// AnalyzerMap is a map that associates keys of a specified type (Key) with Analyzer instances.
	// It is used to store and retrieve Analyzer implementations for various types.
	AnalyzerMap[Key comparable, Input, Output any] map[Key]Analyzer[Input, Output]
)

type (
	AnalyzerFactory[Input, Output any] func(f *obj.FileObj) Analyzer[Input, Output]
	AnalyzeFunc[Input, Output any]     func(ctx context.Context, f *obj.FileObj, i Input) (Output, error)
	SplitterFunc[Input, Output any]    func(ctx context.Context, i Input) AnalyzeFunc[Input, Output]
)

func NewAnalyzer[Input, Output any](
	file *obj.FileObj,
	analyze AnalyzeFunc[Input, Output],
) Analyzer[Input, Output] {
	return &analyzer[Input, Output]{
		file:    file,
		analyze: analyze,
	}
}

type analyzer[Input, Output any] struct {
	file    *obj.FileObj
	analyze AnalyzeFunc[Input, Output]
}

func (a *analyzer[Input, Output]) Analyze(ctx context.Context, i Input) (Output, error) {
	return a.analyze(ctx, a.file, i)
}
