package ray

import (
	"github.com/hashicorp/hcl/v2"
)

var (
	configFileReservedAttributeList = [...]string{}
	configFileReservedBlockList     = [...]string{}
)

var configFileSchema = &hcl.BodySchema{
	Attributes: NewAttributeList()(configFileReservedAttributeList[:]...),
	Blocks: NewBlockList(
		hcl.BlockHeaderSchema{
			Type:       "_",
			LabelNames: []string{},
		},
		hcl.BlockHeaderSchema{
			Type:       "ray",
			LabelNames: []string{},
		},
	)(configFileReservedBlockList[:]...),
}

type File struct {
	Ray *Ray
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
			rayBlock, decodeDiag := DecodeRayBlock(b)
			diagnostics = append(diagnostics, decodeDiag...)
			if diagnostics.HasErrors() {
				return nil, diagnostics
			}

			if file.Ray == nil {
				file.Ray = rayBlock
				continue
			}

			diagnostics = append(diagnostics, file.Ray.Merge(rayBlock)...)
			if diagnostics.HasErrors() {
				return nil, diagnostics
			}

			continue
		}
	}

	return file, diagnostics
}
