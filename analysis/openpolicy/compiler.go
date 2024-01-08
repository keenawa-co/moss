package openpolicy

// type compileFn func(options ...compileOptFn) Compiler
// type compileOptFn func(*ast.Compiler)

// type compiler struct {
// 	Rc    *ast.Compiler
// 	ghash string // group hash
// }

// func (c *compiler) Compile(files map[string]*ast.Module) (*compiler, error) {
// 	c.Rc.Compile(files)
// 	if c.Rc.Failed() {
// 		return nil, c.Rc.Errors
// 	}

// 	return c, nil
// }

// func WithEnablePrintStatements(value bool) compileOptFn {
// 	return func(c *ast.Compiler) {
// 		c.WithEnablePrintStatements(value)
// 	}
// }

// func NewCompiler(options ...compileOptFn) *compiler {
// 	regoCompiler := ast.NewCompiler()

// 	for i := range options {
// 		options[i](regoCompiler)
// 	}

// 	return &compiler{Rc: regoCompiler}
// }
