package ray

import (
	"fmt"

	"github.com/hashicorp/hcl/v2"
)

var (
	rayBlockReservedAttributeList = [...]string{}
	rayBlockReservedBlockList     = [...]string{}
)

var rayBlockSchema = &hcl.BodySchema{
	Attributes: NewAttributeList()(rayBlockReservedAttributeList[:]...),
	Blocks: NewBlockList(
		hcl.BlockHeaderSchema{
			Type:       "_",
			LabelNames: []string{},
		},
		hcl.BlockHeaderSchema{
			Type:       "required_provider",
			LabelNames: []string{"name"},
		},
	)(rayBlockReservedBlockList[:]...),
}

type Ray struct {
	_                 [0]int
	RequiredProviders map[string]*RequiredProvider
}

func (b *Ray) Merge(input *Ray) (diagnostics hcl.Diagnostics) {
	for _, rp := range input.RequiredProviders {
		if _, exists := b.RequiredProviders[rp.Name]; exists {
			diagnostics = append(diagnostics, &hcl.Diagnostic{
				Severity: hcl.DiagWarning,
				Summary:  "Duplicated required provider",
				// TODO: Detail:   fmt.Sprintf("Provider name is invalid. %s", badIdentDetail),
			})
			continue
		}

		b.RequiredProviders[rp.Name] = rp
	}

	return diagnostics
}

func DecodeRayBlock(block *hcl.Block) (ray *Ray, diagnostics hcl.Diagnostics) {
	content, partialContentDiag := block.Body.Content(rayBlockSchema)
	diagnostics = append(diagnostics, partialContentDiag...)

	ray = &Ray{
		RequiredProviders: make(map[string]*RequiredProvider),
	}

	for _, b := range content.Blocks {
		switch b.Type {
		case "required_provider":
			if len(b.Labels) < 1 {
				diagnostics = append(diagnostics, &hcl.Diagnostic{
					Severity: hcl.DiagInvalid,
					Summary:  "Provider name not found",
					Detail:   fmt.Sprintf("The provider name must be specified as the first block label, on line: %d", b.DefRange.Start.Line),
				})
				return nil, diagnostics
			}

			if _, exists := ray.RequiredProviders[b.Labels[0]]; exists {
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
				return nil, diagnostics
			}

			ray.RequiredProviders[b.Labels[0]] = provider
		}
	}

	return ray, diagnostics
}
