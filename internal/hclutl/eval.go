package hclutl

import (
	"github.com/hashicorp/hcl/v2"
	"github.com/hashicorp/hcl/v2/hcldec"
	"github.com/zclconf/go-cty/cty"
)

func (s *Scope) EvalBlock(ctx *hcl.EvalContext, body hcl.Body, spec hcldec.Spec) (val cty.Value, diagnostics hcl.Diagnostics) {
	// ctx := &hcl.EvalContext{
	// 	Variables: map[string]cty.Value{
	// 		"input": cty.ObjectVal(map[string]cty.Value{
	// 			"module_example": cty.StringVal("Example"),
	// 		}),
	// 	},
	// }

	val, diags := hcldec.Decode(body, spec, ctx)
	diagnostics = append(diagnostics, diags...)

	return val, diagnostics
}
