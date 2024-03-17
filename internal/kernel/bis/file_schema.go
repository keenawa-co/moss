package bis

import (
	"fmt"

	"github.com/4rchr4y/goray/internal/hclutl"
	"github.com/hashicorp/hcl/v2"
)

// Reserved for future expansion
var (
	fileReservedAttributeList = [...]string{}
	fileReservedBlockList     = [...]string{
		"package",
		"mod",
		"import",
	}
)

var fileSchema = &hcl.BodySchema{
	Attributes: hclutl.NewAttributeList()(fileReservedAttributeList[:]...),
	Blocks: hclutl.NewBlockList(
		hcl.BlockHeaderSchema{
			Type:       "_",
			LabelNames: []string{},
		},
		hcl.BlockHeaderSchema{
			Type: "component",
			LabelNames: []string{
				"name",
			},
		},
	)(fileReservedBlockList[:]...),
}

type File struct {
	_          [0]int
	Name       string
	Components map[string]*ComponentBlock
}

func DecodeFile(body hcl.Body) (file *File, diagnostics hcl.Diagnostics) {
	content, diags := body.Content(fileSchema)
	diagnostics = append(diagnostics, diags...)
	if diagnostics.HasErrors() {
		return nil, diagnostics
	}

	file = &File{}

	for _, b := range content.Blocks {
		switch b.Type {
		case "component":
			if len(b.Labels) < 1 {
				diagnostics = append(diagnostics, &hcl.Diagnostic{
					Severity: hcl.DiagInvalid,
					Summary:  "Component name not found",
					Detail:   fmt.Sprintf("The component name must be specified as the first block label, on line: %d", b.DefRange.Start.Line),
				})
				return nil, diagnostics
			}

			if _, exists := file.Components[b.Labels[0]]; exists {
				diagnostics = append(diagnostics, &hcl.Diagnostic{
					Severity: hcl.DiagError,
					Summary:  "Duplicated component definition",
					// TODO: Detail:   fmt.Sprintf("Provider name is invalid. %s", badIdentDetail),
				})

				return nil, diagnostics
			}

			diagnostics = append(diagnostics, decodeComponentBlock(file, b)...)
		}
	}

	return file, diagnostics
}

func decodeComponentBlock(file *File, block *hcl.Block) (diagnostics hcl.Diagnostics) {
	b, diags := DecodeComponentBlock(block)
	diagnostics = append(diagnostics, diags...)
	if diagnostics.HasErrors() {
		return diagnostics
	}

	if file.Components == nil {
		file.Components = map[string]*ComponentBlock{
			block.Labels[0]: b,
		}

		return diagnostics
	}

	file.Components[block.Labels[0]] = b
	return diagnostics
}
