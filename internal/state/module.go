package state

import (
	"github.com/hashicorp/hcl/v2"
	"github.com/zclconf/go-cty/cty"
)

type NamedValueType uint

const (
	LET NamedValueType = iota
	PROPS
)

var NamedValueTypeStr = [...]string{
	LET:   "let",
	PROPS: "props",
}

type Module struct {
	address string
	values  map[string]map[string]cty.Value
}

func NewModule(path string) *Module {
	return &Module{
		address: path,
		values: map[string]map[string]cty.Value{
			NamedValueTypeStr[LET]:   make(map[string]cty.Value),
			NamedValueTypeStr[PROPS]: make(map[string]cty.Value),
		},
	}
}

func (m *Module) AppendValue(nvt NamedValueType, name string, value cty.Value) *hcl.Diagnostic {
	// TODO: check is already exists
	switch nvt {
	case LET:
		m.values[NamedValueTypeStr[LET]][name] = value
		return nil
	case PROPS:
		m.values[NamedValueTypeStr[PROPS]][name] = value
		return nil
	default:
		panic("invalid named value type")
	}
}

func (m *Module) Variables() map[string]cty.Value {
	result := make(map[string]cty.Value, len(m.values))

	for k, v := range m.values {
		result[k] = cty.ObjectVal(v)
	}

	return result
}
