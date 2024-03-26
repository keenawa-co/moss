package kernel

import "github.com/zclconf/go-cty/cty"

type NamedValueSet struct {
	variables map[string]cty.Value
	props     map[string]cty.Value
}

func NewNamedValueSet() *NamedValueSet {
	return &NamedValueSet{
		variables: make(map[string]cty.Value),
		props:     make(map[string]cty.Value),
	}
}

func (nvs *NamedValueSet) SetVariable(name string, value cty.Value) {
	nvs.variables[name] = value
}

func (nvs *NamedValueSet) SetProperty(name string, value cty.Value) {
	nvs.props[name] = value
}
