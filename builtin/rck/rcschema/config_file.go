package rcschema

import (
	"fmt"

	"github.com/4rchr4y/goray/pkg/hclutil"
	"github.com/hashicorp/hcl/v2"
)

// Reserved for future expansion
var (
	configFileReservedAttributeList = [...]string{}
	configFileReservedBlockList     = [...]string{
		"package",
		"mod",
		"import",
		"data",
		"addon",
	}
)

var configFileSchema = &hcl.BodySchema{
	Attributes: hclutil.NewAttributeList()(configFileReservedAttributeList[:]...),
	Blocks: hclutil.NewBlockList(
		hcl.BlockHeaderSchema{
			Type:       "_",
			LabelNames: []string{},
		},
		hcl.BlockHeaderSchema{
			Type:       "ray",
			LabelNames: []string{},
		},
		hcl.BlockHeaderSchema{
			Type: "provider",
			LabelNames: []string{
				"name",
			},
		},
	)(configFileReservedBlockList[:]...),
}

type File struct {
	Ray       *Ray
	Providers map[string]*Provider
}

func DecodeFile(body hcl.Body) (file *File, diagnostics hcl.Diagnostics) {
	content, diags := body.Content(configFileSchema)
	diagnostics = append(diagnostics, diags...)
	if diagnostics.HasErrors() {
		return nil, diagnostics
	}

	file = &File{}

	for _, b := range content.Blocks {
		switch b.Type {
		case "ray":
			diagnostics = append(diagnostics, decodeRayBlock(file, b)...)

		case "provider":
			if len(b.Labels) < 1 {
				diagnostics = append(diagnostics, &hcl.Diagnostic{
					Severity: hcl.DiagInvalid,
					Summary:  "Provider name not found",
					Detail:   fmt.Sprintf("The provider name must be specified as the first block label, on line: %d", b.DefRange.Start.Line),
				})
				return nil, diagnostics
			}

			if _, exists := file.Providers[b.Labels[0]]; exists {
				diagnostics = append(diagnostics, &hcl.Diagnostic{
					Severity: hcl.DiagError,
					Summary:  "Duplicated provider",
					// TODO: Detail:   fmt.Sprintf("Provider name is invalid. %s", badIdentDetail),
				})

				return nil, diagnostics
			}

			diagnostics = append(diagnostics, decodeProviderBlock(file, b)...)
		}
	}

	return file, diagnostics
}

func decodeProviderBlock(file *File, block *hcl.Block) (diagnostics hcl.Diagnostics) {
	b, diags := DecodeProviderBlock(block)
	diagnostics = append(diagnostics, diags...)
	if diagnostics.HasErrors() {
		return diagnostics
	}

	if file.Providers == nil {
		file.Providers = map[string]*Provider{
			block.Labels[0]: b,
		}

		return diagnostics
	}

	file.Providers[block.Labels[0]] = b
	return diagnostics
}

func decodeRayBlock(file *File, block *hcl.Block) (diagnostics hcl.Diagnostics) {
	b, diags := DecodeRayBlock(block)
	diagnostics = append(diagnostics, diags...)
	if diagnostics.HasErrors() {
		return diagnostics
	}

	if file.Ray == nil {
		file.Ray = b
		return diagnostics
	}

	diagnostics = append(diagnostics, file.Ray.Merge(b)...)
	if diagnostics.HasErrors() {
		return diagnostics
	}

	return diagnostics
}
