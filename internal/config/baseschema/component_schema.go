package baseschema

import (
	"fmt"

	"github.com/4rchr4y/goray/internal/hclutl"
	"github.com/4rchr4y/goray/internal/kernel/hcllang"
	"github.com/hashicorp/hcl/v2"
	"github.com/hashicorp/hcl/v2/gohcl"
	"github.com/hashicorp/hcl/v2/hclsyntax"
)

// Reserved for future expansion
var (
	componentBlockReservedAttributeList = [...]string{}
	componentBlockReservedBlockList     = [...]string{}
)

var providerBlockSchema = &hcl.BodySchema{
	Attributes: hclutl.NewAttributeList(
		hcl.AttributeSchema{
			Name:     "version",
			Required: false,
		},
	)(componentBlockReservedAttributeList[:]...),
	Blocks: hclutl.NewBlockList()(componentBlockReservedBlockList[:]...),
}

type Component struct {
	_       [0]int
	Name    string
	Version string
	Config  hcl.Body
}

func DecodeComponentBlock(block *hcl.Block) (component *Component, diagnostics hcl.Diagnostics) {
	content, body, diagnostics := block.Body.PartialContent(providerBlockSchema)
	if diagnostics.HasErrors() {
		return nil, diagnostics
	}

	// existence of a label is checked when this block was detected
	if !hclsyntax.ValidIdentifier(block.Labels[0]) {
		diagnostics = append(diagnostics, &hcl.Diagnostic{
			Severity: hcl.DiagError,
			Summary:  "Invalid component name",
			Detail:   fmt.Sprintf("Component name is invalid. %s", hcllang.BadIdentDetail),
			Subject:  &block.LabelRanges[0],
		})
	}

	component = &Component{
		Name:   block.Labels[0],
		Config: body,
	}

	if attr, exists := content.Attributes["version"]; exists {
		diags := gohcl.DecodeExpression(attr.Expr, nil, &component.Version)
		diagnostics = append(diagnostics, diags...)
		if diagnostics.HasErrors() {
			return nil, diagnostics
		}

		// TODO: source validation
	}

	return component, diagnostics
}
