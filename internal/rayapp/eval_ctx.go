package rayapp

import (
	"github.com/4rchr4y/goray/diag"
	"github.com/4rchr4y/goray/internal/hcllang"
	"github.com/4rchr4y/goray/internal/schematica"
	"github.com/hashicorp/hcl/v2"
	"github.com/zclconf/go-cty/cty"
)

type AppEvalContext struct {
	Evaluator *Evaluator
}

func (ctx *AppEvalContext) EvaluateBlock(body hcl.Body, schema *schematica.Block, selector hcllang.Data) (cty.Value, hcl.Body, diag.DiagnosticSet) {
	var diags diag.DiagnosticSet

	scope := hcllang.NewScope()
	scope.Data = selector

	val, diags := scope.EvalBlock(body, schema)
	diags = diags.Append(diags)

	return val, body, diags
}
