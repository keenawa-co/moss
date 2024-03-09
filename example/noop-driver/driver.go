package noop_driver

import (
	"github.com/4rchr4y/goray/interface/driver"
	"github.com/4rchr4y/goray/internal/schematica"
	"github.com/zclconf/go-cty/cty"
)

type NoopDriver struct{}

func Driver() driver.Interface {
	return &NoopDriver{}
}

func (s *NoopDriver) DescribeSchema() *driver.DescribeSchemaOutput {
	return &driver.DescribeSchemaOutput{
		Schema: &driver.Schema{
			Version: 1,
			Root: &schematica.Block{
				Attributes: map[string]*schematica.Attribute{
					"value": {
						Optional: true,
						Type:     cty.String,
					},
				},
				Description: "Hello, Ray from 'noop-driver'!",
			},
		},
	}
}

func (s *NoopDriver) ReadResource(*driver.ReadResourceInput) *driver.ReadResourceOutput {
	return &driver.ReadResourceOutput{}
}

func (s *NoopDriver) Stop() error {
	return nil
}

func (s *NoopDriver) Shutdown() error {
	return nil
}
