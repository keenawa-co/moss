package compass

import (
	"go/ast"

	"github.com/4rchr4y/go-compass/obj"
)

type Picker interface {
	Pick(state *State, node ast.Node) (obj.Object, error)
}

type (
	PickerFactory func() Picker
	PickFunc      func(s *State, node ast.Node) (obj.Object, error)
)

func NewPicker(pick PickFunc) Picker {
	return &picker{
		pick: pick,
	}
}

type picker struct {
	pick PickFunc
}

func (a *picker) Pick(s *State, n ast.Node) (obj.Object, error) {
	return a.pick(s, n)
}
