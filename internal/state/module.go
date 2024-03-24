package state

import (
	"github.com/zclconf/go-cty/cty"
)

type Module struct {
	Address      string
	OutputValues map[string]cty.Value
	PropsValues  map[string]map[string]cty.Value
}
