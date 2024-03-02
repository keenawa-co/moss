package ray

import (
	"fmt"

	"github.com/hashicorp/hcl/v2"
	"github.com/hashicorp/hcl/v2/gohcl"
	"github.com/hashicorp/hcl/v2/hclsyntax"
)

var (
	requiredProviderBlockReservedAttributeList = [...]string{
		"aliases",
	}
	requiredProviderBlockReservedBlockList = [...]string{}
)

var requiredProviderBlockSchema = &hcl.BodySchema{
	Attributes: NewAttributeList(
		hcl.AttributeSchema{
			Name:     "source",
			Required: true,
		},
		hcl.AttributeSchema{
			Name:     "version",
			Required: true,
		},
	)(requiredProviderBlockReservedAttributeList[:]...),
	Blocks: NewBlockList(
		hcl.BlockHeaderSchema{Type: "_"},
	)(requiredProviderBlockReservedBlockList[:]...),
}

type RequiredProvider struct {
	_       [0]int
	Name    string
	Source  string
	Version string
	Body    hcl.Body
	Content *hcl.BodyContent
}

func DecodeRequiredProviderBlock(block *hcl.Block) (provider *RequiredProvider, diagnostics hcl.Diagnostics) {
	content, body, partialContentDiag := block.Body.PartialContent(requiredProviderBlockSchema)
	diagnostics = append(diagnostics, partialContentDiag...)

	if !hclsyntax.ValidIdentifier(block.Labels[0]) {
		diagnostics = append(diagnostics, &hcl.Diagnostic{
			Severity: hcl.DiagError,
			Summary:  "Invalid provider name",
			Detail:   fmt.Sprintf("Provider name is invalid. %s", badIdentDetail),
		})
	}

	provider = &RequiredProvider{
		Name:    block.Labels[0],
		Content: content,
		Body:    body,
	}

	if attr, exists := content.Attributes["source"]; exists {
		diags := gohcl.DecodeExpression(attr.Expr, nil, &provider.Source)
		diagnostics = append(diagnostics, diags...)

		// TODO: source validation
	}

	if attr, exists := content.Attributes["version"]; exists {
		diags := gohcl.DecodeExpression(attr.Expr, nil, &provider.Source)
		diagnostics = append(diagnostics, diags...)

		// TODO: source validation
	}

	return provider, diagnostics
}

func NewAttributeList(attributes ...hcl.AttributeSchema) func(reserved ...string) []hcl.AttributeSchema {
	return func(reserved ...string) []hcl.AttributeSchema {
		for i := range reserved {
			attributes = append(attributes, hcl.AttributeSchema{
				Name:     reserved[i],
				Required: false,
			})
		}

		return attributes
	}
}

func NewBlockList(blocks ...hcl.BlockHeaderSchema) func(reserved ...string) []hcl.BlockHeaderSchema {
	return func(reserved ...string) []hcl.BlockHeaderSchema {
		for i := range reserved {
			blocks = append(blocks, hcl.BlockHeaderSchema{
				Type: reserved[i],
			})
		}

		return blocks
	}
}
