package hclutl

import (
	"github.com/hashicorp/hcl/v2"
	"github.com/hashicorp/hcl/v2/hcldec"
	"github.com/zclconf/go-cty/cty"
)

func (s *Scope) EvalBlock(body hcl.Body, spec hcldec.Spec) (val cty.Value, diagnostics hcl.Diagnostics) {
	ctx := &hcl.EvalContext{}
	val, diags := hcldec.Decode(body, spec, ctx)
	diagnostics = append(diagnostics, diags...)

	return val, diagnostics
}
