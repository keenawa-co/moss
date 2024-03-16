package dummy_component

import (
	"fmt"

	"github.com/4rchr4y/godevkit/v3/must"
	"github.com/4rchr4y/goray/diag"
	"github.com/4rchr4y/goray/interface/component"
	"github.com/4rchr4y/goray/internal/proto/convert"
	"github.com/4rchr4y/goray/internal/schematica"
	"github.com/hashicorp/hcl/v2/hcldec"
	"github.com/zclconf/go-cty/cty"
)

var componentSchema = schematica.Block{
	Attributes: map[string]*schematica.Attribute{
		"value": {
			Optional: true,
			Type:     cty.String,
		},
	},
	Description: "Hello, Ray from 'dummy-component'!",
}

type DummyComponent struct {
	value string
}

func Component() component.Interface {
	return &DummyComponent{}
}

func (s *DummyComponent) Configure(input *component.ConfigureInput) *component.ConfigureOutput {
	output := new(component.ConfigureOutput)
	spec := must.Must(schematica.DecodeBlock(&componentSchema))
	decoded, err := convert.DecodeValue(input.MessagePack, hcldec.ImpliedType(spec))
	if err != nil {
		output.Diagnostics = output.Diagnostics.Append(diag.NewNativeError(err))
		return output
	}

	s.value = decoded.GetAttr("value").AsString()

	fmt.Printf("\n----------------- %s -----------------\n", s.value)

	return new(component.ConfigureOutput)
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
			Root:    &componentSchema,
		},
	}
}

func (s *DummyComponent) Stop() error {
	return nil
}

func (s *DummyComponent) Shutdown() error {
	return nil
}
