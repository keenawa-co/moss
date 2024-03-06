package rcschema

import (
	"fmt"

	"github.com/4rchr4y/goray/kernel/hcllang"
	"github.com/hashicorp/hcl/v2"
	"github.com/hashicorp/hcl/v2/hclsyntax"
)

// Reserved for future expansion
var (
	providerBlockReservedAttributeList = [...]string{}
	providerBlockReservedBlockList     = [...]string{}
)

var providerBlockSchema = &hcl.BodySchema{
	Attributes: NewAttributeList()(providerBlockReservedAttributeList[:]...),
	Blocks:     NewBlockList()(providerBlockReservedBlockList[:]...),
}

type Provider struct {
	_       [0]int
	Name    string
	Content *hcl.BodyContent
}

func DecodeProviderBlock(block *hcl.Block) (provider *Provider, diagnostics hcl.Diagnostics) {
	content, _, partialContentDiag := block.Body.PartialContent(providerBlockSchema)
	diagnostics = append(diagnostics, partialContentDiag...)

	// existence of a label is checked when a block is detected
	if !hclsyntax.ValidIdentifier(block.Labels[0]) {
		diagnostics = append(diagnostics, &hcl.Diagnostic{
			Severity: hcl.DiagError,
			Summary:  "Invalid provider name",
			Detail:   fmt.Sprintf("Provider name is invalid. %s", hcllang.BadIdentDetail),
		})
	}

	provider = &Provider{
		Name:    block.Labels[0],
		Content: content,
	}

	return provider, diagnostics
}
