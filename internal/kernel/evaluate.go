package kernel

// type Module struct {
// 	NamedValues *hcllang.NamedValueSet
// }

// type State struct {
// 	Modules map[string]*Module
// }

// type Evaluate struct {
// 	RootConf *config.Config
// 	Scope    *hcllang.Scope
// 	State    *State
// }

// func NewEvaluate(conf *config.Config, scope *hcllang.Scope) *Evaluate {
// 	return &Evaluate{
// 		RootConf: conf,
// 		Scope:    scope,
// 		State: &State{
// 			Modules: make(map[string]*Module),
// 		},
// 	}
// }

// func (e *Evaluate) ExpandVariables() (*State, hcl.Diagnostics) {
// 	ctx := &hcl.EvalContext{
// 		Functions: e.Scope.Functions(),
// 		Variables: make(map[string]cty.Value),
// 	}

// 	return e.expandVariables(ctx, e.RootConf)
// }

// func (e *Evaluate) expandVariables(ctx *hcl.EvalContext, conf *config.Config) (s *State, diagnostics hcl.Diagnostics) {
// 	mod := NewModule()

// 	for name, variable := range conf.Module.Variables {
// 		mod.NamedValues.SetVariable(name, variable.Default)
// 	}

// 	propsAttrsValue, diags := e.expandModuleProps(ctx, conf)
// 	diagnostics = append(diagnostics, diags...)

// 	if conf.Module.Header != nil {
// 		for name, variable := range conf.Module.Header.Variables {
// 			if val, exists := propsAttrsValue[name]; exists {
// 				mod.NamedValues.SetProperty(name, val)
// 			} else {
// 				mod.NamedValues.SetProperty(name, variable.Default)
// 			}
// 		}
// 	}

// 	e.State.Modules[conf.Path] = mod

// 	for _, childConf := range conf.Children {
// 		s, diags := e.expandVariables(ctx, childConf)
// 		diagnostics = append(diagnostics, diags...)
// 		e.State = s
// 	}

// 	return e.State, diagnostics
// }

// func (e *Evaluate) expandModuleProps(ctx *hcl.EvalContext, conf *config.Config) (propsAttrsValue map[string]cty.Value, diagnostics hcl.Diagnostics) {
// 	propsSchema, propsMeta := conf.Module.PropsSchema()
// 	if propsSchema == nil {
// 		return nil, diagnostics
// 	}

// 	propsAttrsValue = make(map[string]cty.Value, propsMeta.AttributesSize)
// 	content, diags := conf.Parent.Module.Includes.Modules[conf.Path].Config.Content(propsSchema)
// 	diagnostics = append(diagnostics, diags...)

// 	for _, v := range content.Attributes {
// 		val, _ := v.Expr.Value(&hcl.EvalContext{
// 			Functions: ctx.Functions,
// 			Variables: e.State.Modules[conf.Parent.Path].NamedValues.GetVariableSet(),
// 		})

// 		propsAttrsValue[v.Name] = val
// 	}

// 	return propsAttrsValue, diagnostics
// }
