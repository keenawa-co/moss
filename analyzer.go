package compass

import "github.com/4rchr4y/go-compass/state"

type Analyzer[Input, Output any] interface {
	Analyze(state *state.State, input Input) (Output, error)
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
	AnalyzerFactory[Input, Output any] func() Analyzer[Input, Output]
	AnalyzeFunc[Input, Output any]     func(s *state.State, i Input) (Output, error)
)

func NewAnalyzer[Input, Output any](analyze AnalyzeFunc[Input, Output]) Analyzer[Input, Output] {
	return &analyzer[Input, Output]{
		analyze: analyze,
	}
}

type analyzer[Input, Output any] struct {
	analyze AnalyzeFunc[Input, Output]
}

func (a *analyzer[Input, Output]) Analyze(s *state.State, i Input) (Output, error) {
	return a.analyze(s, i)
}
