package hclwrap

import (
	"github.com/hashicorp/hcl/v2"
	"github.com/hashicorp/hcl/v2/hcldec"
	"github.com/zclconf/go-cty/cty"
)

func (s *Scope) EvalBlock(body hcl.Body, spec hcldec.Spec) (val cty.Value, diagnostics hcl.Diagnostics) {
	// spec, err := schematica.DecodeBlock(block)
	// if err != nil {
	// 	diagnostics = append(diagnostics, &hcl.Diagnostic{
	// 		Severity: hcl.DiagError,
	// 		Summary:  err.Error(),
	// 		// TODO: Detail:   ,
	// 	})
	// }

	ctx := &hcl.EvalContext{}
	val, diags := hcldec.Decode(body, spec, ctx)
	diagnostics = append(diagnostics, diags...)

	return val, diagnostics
}
