package baseschema

import (
	"github.com/4rchr4y/goray/internal/hclutl"
	"github.com/hashicorp/hcl/v2"
	"github.com/hashicorp/hcl/v2/gohcl"
)

// Reserved for future expansion
var (
	includeModuleBlockReservedAttributeList = [...]string{}
	includeModuleBlockReservedBlockList     = [...]string{
		"lifecycle",
	}
)

var includeModuleBlockSchema = &hcl.BodySchema{
	Attributes: hclutl.NewAttributeList(
		hcl.AttributeSchema{
			Name:     "source",
			Required: true,
		},
	)(includeModuleBlockReservedAttributeList[:]...),
	Blocks: hclutl.NewBlockList()(includeModuleBlockReservedBlockList[:]...),
}

type IncludeModule struct {
	Name      string
	Source    string
	Config    hcl.Body
	DeclRange hcl.Range
}

func DecodeIncludeModuleBlock(block *hcl.Block) (b *IncludeModule, diagnostics hcl.Diagnostics) {
	content, body, diagnostics := block.Body.PartialContent(includeModuleBlockSchema)
	if diagnostics.HasErrors() {
		return nil, diagnostics
	}

	b = &IncludeModule{
		Name:      block.Labels[0], // label presence was verified upon block detection
		Config:    body,
		DeclRange: block.DefRange,
	}

	if attr, exists := content.Attributes["source"]; exists {
		diags := gohcl.DecodeExpression(attr.Expr, nil, &b.Source)
		diagnostics = append(diagnostics, diags...)
		if diagnostics.HasErrors() {
			return nil, diagnostics
		}
	}

	return b, diagnostics
}
