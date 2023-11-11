package picker

import (
	"go/ast"

	"github.com/4rchr4y/go-compass/state"
)

type Picker[Output any] interface {
	Pick(state *state.State, node ast.Node) (Output, error)
}

type (
	PickerFactory[Output any] func() Picker[Output]
	PickFunc[Output any]      func(s *state.State, node ast.Node) (Output, error)
)

func NewPicker[Output any](pick PickFunc[Output]) Picker[Output] {
	return &picker[Output]{
		pick: pick,
	}
}

type picker[Output any] struct {
	pick PickFunc[Output]
}

func (a *picker[Output]) Pick(s *state.State, n ast.Node) (Output, error) {
	return a.pick(s, n)
}
