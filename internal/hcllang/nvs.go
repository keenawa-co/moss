package hcllang

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

func (nvs *NamedValueSet) SetLetVariableValue(name string, value cty.Value) {
	nvs.variables[name] = value
}

func (nvs *NamedValueSet) SetPropVariableValue(name string, value cty.Value) {
	nvs.props[name] = value
}

func (nvs *NamedValueSet) GetLetVariableValue(addr string) cty.Value {
	return nvs.props[addr]
}

func (nvs *NamedValueSet) GetPropVariableValue(addr string) cty.Value {
	return nvs.props[addr]
}

// func (nvs *NamedValueSet) GetVariableSet() map[string]cty.Value {
// 	return map[string]cty.Value{
// 		"let":   cty.ObjectVal(nvs.variables),
// 		"props": cty.ObjectVal(nvs.props),
// 	}
// }
