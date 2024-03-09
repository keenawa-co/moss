package noop_driver

import (
	"github.com/4rchr4y/goray/interface/driver"
	"github.com/4rchr4y/goray/internal/schematica"
	"github.com/zclconf/go-cty/cty"
)

type NoopDriverServer struct{}

func Driver() driver.Interface {
	return &NoopDriverServer{}
}

func (s *NoopDriverServer) DescribeSchema() *driver.DescribeSchemaOutput {
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
				Description: "Hello, Ray!",
			},
		},
	}
}

func (s *NoopDriverServer) ReadResource(*driver.ReadResourceInput) *driver.ReadResourceOutput {
	return &driver.ReadResourceOutput{}
}

func (s *NoopDriverServer) Stop() error {
	return nil
}

func (s *NoopDriverServer) Shutdown() error {
	return nil
}
