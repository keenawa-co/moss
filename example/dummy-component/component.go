package dummy_component

import (
	"fmt"

	"github.com/4rchr4y/goray/interface/component"
	"github.com/4rchr4y/goray/internal/schematica"
	"github.com/zclconf/go-cty/cty"
)

type DummyComponent struct {
	// value string
}

func Component() component.Interface {
	return &DummyComponent{}
}

func (s *DummyComponent) Configure(input *component.ConfigureInput) (*component.ConfigureOutput, error) {
	fmt.Println(input.Schema.Attributes["value"])
	return new(component.ConfigureOutput), nil
}

func (s *DummyComponent) Heartbeat() *component.HeartbeatOutput {
	return &component.HeartbeatOutput{
		Status: "OK",
	}
}

func (s *DummyComponent) DescribeSchema() *component.DescribeSchemaOutput {
	return &component.DescribeSchemaOutput{
		Schema: &component.Schema{
			Version: 1,
			Root: &schematica.Block{
				Attributes: map[string]*schematica.Attribute{
					"value": {
						Optional: true,
						Type:     cty.String,
					},
				},
				Description: "Hello, Ray from 'dummy-component'!",
			},
		},
	}
}

func (s *DummyComponent) Stop() error {
	return nil
}

func (s *DummyComponent) Shutdown() error {
	return nil
}
