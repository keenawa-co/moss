package hclwrap

import (
	"github.com/4rchr4y/goray/internal/schematica"
	"github.com/hashicorp/hcl/v2"
	"github.com/hashicorp/hcl/v2/hcldec"
	"github.com/zclconf/go-cty/cty"
)

func (s *Scope) EvalBlock(body hcl.Body, block *schematica.Block) (v cty.Value, diagnostics hcl.Diagnostics) {
	spec, err := schematica.DecodeBlock(block)
	if err != nil {
		diagnostics = append(diagnostics, &hcl.Diagnostic{
			Severity: hcl.DiagError,
			Summary:  err.Error(),
			// TODO: Detail:   ,
		})
	}

	ctx := &hcl.EvalContext{}
	val, diags := hcldec.Decode(body, spec, ctx)
	diagnostics = append(diagnostics, diags...)

	return val, diagnostics
}
