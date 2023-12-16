package openpolicy

import (
	"github.com/open-policy-agent/opa/ast"
)

type CompileOptFn func(*ast.Compiler)

func Compile(parsed map[string]*ast.Module, options ...CompileOptFn) (*ast.Compiler, error) {
	compiler := ast.NewCompiler()

	for i := 0; i < len(options); i++ {
		options[i](compiler)
	}

	compiler.Compile(parsed)
	if compiler.Failed() {
		return nil, compiler.Errors

	}

	return compiler, nil
}

func WithEnablePrintStatements(value bool) CompileOptFn {
	return func(c *ast.Compiler) {
		c.WithEnablePrintStatements(value)
	}
}
