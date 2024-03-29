package hcllang

import "github.com/hashicorp/hcl/v2"

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
