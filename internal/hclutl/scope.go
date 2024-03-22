package hclutl

import (
	"strings"

	"github.com/4rchr4y/goray/internal/hcllang"
	"github.com/zclconf/go-cty/cty/function"
)

type Scope struct {
	functions map[string]function.Function
}

func NewScope() *Scope {
	scope := &Scope{
		functions: make(map[string]function.Function, len(hcllang.BuiltinFunctionList)*2),
	}

	builder := strings.Builder{}

	for ident, fn := range hcllang.BuiltinFunctionList {
		builder.Reset()

		builder.WriteString(ident.Namespace())
		builder.WriteString(hcllang.IdentSeparator)
		builder.WriteString(ident.Name())

		scope.functions[ident.Name()] = fn
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
