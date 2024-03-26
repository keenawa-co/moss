package confeval

// func BuildEvalContextGraph(scope *hclutl.Scope, conf *config.Config) *state.State {
// 	ctx := &hcl.EvalContext{
// 		Functions: scope.Functions(),
// 		Variables: make(map[string]cty.Value),
// 	}

// 	s := &state.State{
// 		Modules: make(map[string]*state.Module),
// 	}

// 	return evalModule(ctx, conf, s)
// }

// func evalModule(ctx *hcl.EvalContext, conf *config.Config, s *state.State) *state.State {
// 	m := state.NewModule(conf.Path)

// 	for name, let := range conf.Module.Variables {
// 		_ = m.AppendValue(state.LET, name, let.Default)
// 	}

// 	propsSchema := modulePropsSchema(conf.Module)
// 	propsValue := map[string]cty.Value{}

// 	if propsSchema != nil {
// 		includedPropsDecl := conf.Parent.Module.Includes.Modules[conf.Path].Config
// 		content, _ := includedPropsDecl.Content(propsSchema)

// 		for _, v := range content.Attributes {
// 			val, _ := v.Expr.Value(&hcl.EvalContext{
// 				Functions: ctx.Functions,
// 				Variables: s.Modules[conf.Parent.Path].Variables(),
// 			})

// 			propsValue[v.Name] = val
// 		}
// 	}

// 	if conf.Module.Header != nil {
// 		for name, let := range conf.Module.Header.Variables {
// 			if val, exists := propsValue[name]; exists {
// 				_ = m.AppendValue(state.PROPS, name, val)
// 			} else {
// 				_ = m.AppendValue(state.PROPS, name, let.Default)
// 			}
// 		}
// 	}

// 	s.Modules[conf.Path] = m

// 	for _, c := range conf.Children {
// 		s = evalModule(ctx, c, s)
// 	}

// 	return s
// }

// func modulePropsSchema(module *config.Module) (s *hcl.BodySchema) {
// 	if module.Header == nil {
// 		return nil
// 	}

// 	s = new(hcl.BodySchema)

// 	for _, v := range module.Header.Variables {
// 		s.Attributes = append(s.Attributes, hcl.AttributeSchema{
// 			Name:     v.Name,
// 			Required: v.Nullable,
// 		})
// 	}

// 	return s
// }
