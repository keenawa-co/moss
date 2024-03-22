package confeval

import (
	"github.com/4rchr4y/goray/internal/config"
	"github.com/4rchr4y/goray/internal/hclutl"
	"github.com/hashicorp/hcl/v2"
	"github.com/zclconf/go-cty/cty"
)

type EvalContextGraph struct {
}

func BuildEvalContextGraph(scope *hclutl.Scope, conf *config.Config) *hcl.EvalContext {
	ctx := &hcl.EvalContext{
		Functions: scope.Functions(),
		Variables: make(map[string]cty.Value),
	}

	return evalModule(ctx, conf)
}

func evalModule(ctx *hcl.EvalContext, conf *config.Config) *hcl.EvalContext {

	for name, let := range conf.Module.Variables {
		ctx.Variables[name] = let.Default
	}

	if conf.Module.Header != nil {
		inputValueObj := map[string]cty.Value{}
		for name, let := range conf.Module.Header.Variables {
			inputValueObj[name] = let.Default
		}
		if ctx.Variables == nil {
			ctx.Variables = make(map[string]cty.Value)
		}

		ctx.Variables["input"] = cty.ObjectVal(inputValueObj)

	}

	for _, c := range conf.Children {
		evalModule(ctx.NewChild(), c)
	}

	return ctx
}
