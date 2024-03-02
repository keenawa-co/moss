package ray

import (
	"fmt"

	"github.com/hashicorp/hcl/v2"
)

var (
	requiredBlockReservedAttributeList = [...]string{}
	requiredBlockReservedBlockList     = [...]string{}
)

var requiredBlockSchema = &hcl.BodySchema{
	Attributes: NewAttributeList()(requiredBlockReservedAttributeList[:]...),
	Blocks: NewBlockList(
		hcl.BlockHeaderSchema{
			Type: "_",
		},
		hcl.BlockHeaderSchema{
			Type:       "provider",
			LabelNames: []string{"name"},
		},
	)(requiredBlockReservedBlockList[:]...),
}

type Required struct {
	Providers map[string]*RequiredProvider
	Body      hcl.Body
	Content   *hcl.BodyContent
}

func DecodeRequiredBlock(block *hcl.Block) (required *Required, diagnostics hcl.Diagnostics) {
	content, body, partialContentDiag := block.Body.PartialContent(requiredBlockSchema)
	diagnostics = append(diagnostics, partialContentDiag...)

	required = &Required{
		Content:   content,
		Body:      body,
		Providers: make(map[string]*RequiredProvider),
	}

	for _, b := range content.Blocks {
		switch b.Type {
		case "provider":
			if len(b.Labels) < 1 {
				diagnostics = append(diagnostics, &hcl.Diagnostic{
					Severity: hcl.DiagInvalid,
					Summary:  "Provider name not found",
					Detail:   fmt.Sprintf("The provider name must be specified as the first block label, on line: %d", block.DefRange.Start.Line),
				})
				return nil, diagnostics
			}

			if _, exists := required.Providers[b.Labels[0]]; exists {
				diagnostics = append(diagnostics, &hcl.Diagnostic{
					Severity: hcl.DiagWarning,
					Summary:  "Duplicated provider",
					// TODO: Detail:   fmt.Sprintf("Provider name is invalid. %s", badIdentDetail),
				})
				continue
			}

			provider, decodeDiag := DecodeRequiredProviderBlock(b)
			if decodeDiag.HasErrors() {
				diagnostics = append(diagnostics, decodeDiag...)
				continue
			}

			required.Providers[b.Labels[0]] = provider
		}
	}

	return required, diagnostics
}
