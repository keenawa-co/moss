package baseschema

import (
	"github.com/4rchr4y/goray/internal/hclutl"
	"github.com/hashicorp/hcl/v2"
	"github.com/hashicorp/hcl/v2/gohcl"
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

	component = &Component{
		Name:   block.Labels[0], // label presence was verified upon block detection
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
