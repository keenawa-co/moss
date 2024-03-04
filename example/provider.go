package example

import (
	"github.com/4rchr4y/goray/interface/provider"
	"github.com/4rchr4y/goray/internal/schematica"
	"github.com/zclconf/go-cty/cty"
)

type FirstProvider struct {
	schema *provider.DescribeSchemaOutput
}

func NewFirstProvider() provider.Interface {
	return &FirstProvider{
		schema: &provider.DescribeSchemaOutput{
			Schema: &schematica.Block{
				Attributes: map[string]*schematica.Attribute{
					"id": {
						Optional: true,
						Type:     cty.String,
					},
					"value": {
						Optional: true,
						Type:     cty.String,
					},
				},
			},
		},
	}
}

func (p *FirstProvider) DescribeSchema() *provider.DescribeSchemaOutput {
	return p.schema
}

func (p *FirstProvider) ReadResource(*provider.ReadResourceInput) *provider.ReadResourceOutput {
	return nil
}

func (p *FirstProvider) Stop() error {
	return nil
}

func (p *FirstProvider) Shutdown() error {
	return nil
}
