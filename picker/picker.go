package picker

import (
	"go/ast"

	"github.com/4rchr4y/go-compass/obj"
	"github.com/4rchr4y/go-compass/state"
)

type Picker interface {
	Pick(state *state.State, node ast.Node) (obj.Object, error)
}

type (
	PickerFactory func() Picker
	PickFunc      func(s *state.State, node ast.Node) (obj.Object, error)
)

func NewPicker[Output any](pick PickFunc) Picker {
	return &picker{
		pick: pick,
	}
}

type picker struct {
	pick PickFunc
}

func (a *picker) Pick(s *state.State, n ast.Node) (obj.Object, error) {
	return a.pick(s, n)
}
