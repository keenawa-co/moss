package bis

import (
	"fmt"

	"github.com/4rchr4y/goray/lib/hcllang"
	"github.com/4rchr4y/goray/pkg/hclutil"
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
	Attributes: hclutil.NewAttributeList(
		hcl.AttributeSchema{
			Name:     "version",
			Required: false,
		},
	)(componentBlockReservedAttributeList[:]...),
	Blocks: hclutil.NewBlockList()(componentBlockReservedBlockList[:]...),
}

type ComponentBlock struct {
	_       [0]int
	Name    string
	Version string
	Content *hcl.BodyContent
}

func DecodeComponentBlock(block *hcl.Block) (componentBlock *ComponentBlock, diagnostics hcl.Diagnostics) {
	content, _, partialContentDiag := block.Body.PartialContent(providerBlockSchema)
	diagnostics = append(diagnostics, partialContentDiag...)

	// existence of a label is checked when a block is detected
	if !hclsyntax.ValidIdentifier(block.Labels[0]) {
		diagnostics = append(diagnostics, &hcl.Diagnostic{
			Severity: hcl.DiagError,
			Summary:  "Invalid provider name",
			Detail:   fmt.Sprintf("Component name is invalid. %s", hcllang.BadIdentDetail),
		})
	}

	componentBlock = &ComponentBlock{
		Name:    block.Labels[0],
		Content: content,
	}

	if attr, exists := content.Attributes["version"]; exists {
		diags := gohcl.DecodeExpression(attr.Expr, nil, &componentBlock.Version)
		diagnostics = append(diagnostics, diags...)
		if diagnostics.HasErrors() {
			return nil, diagnostics
		}

		// TODO: source validation
	}

	return componentBlock, diagnostics
}
