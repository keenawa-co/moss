package hcllang

import (
	"strings"

	"github.com/zclconf/go-cty/cty/function"
)

type Scope struct {
	functions map[string]function.Function
}

func NewScope() *Scope {
	scope := &Scope{
		functions: make(map[string]function.Function, len(builtinFunctionList)*2),
	}

	builder := strings.Builder{}

	for ident, fn := range builtinFunctionList {
		builder.Reset()

		builder.WriteString(ident.namespace)
		builder.WriteString(IdentSeparator)
		builder.WriteString(ident.name)

		scope.functions[ident.name] = fn
		scope.functions[builder.String()] = fn
	}

	return scope
}

func (ps *Scope) Functions() map[string]function.Function {
	if ps.functions != nil {
		return ps.functions
	}

	panic("functions have not been defined")
}
