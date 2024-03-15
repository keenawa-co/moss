package component

import (
	"github.com/hashicorp/hcl/v2"
)

type Interface interface {
	Heartbeat() *HeartbeatOutput
	Configure(*ConfigureInput) *ConfigureOutput
	DescribeSchema() *DescribeSchemaOutput
	Stop() error
	Shutdown() error
}

type (
	HeartbeatOutput struct {
		Status string
		Error  error
	}
)

func (o *HeartbeatOutput) WithError(err error) *HeartbeatOutput {
	o.Error = err
	return o
}

type (
	ConfigureInput struct {
		MessagePack []byte
	}

	ConfigureOutput struct {
		Error error
	}
)

func (o *ConfigureOutput) WithError(err error) *ConfigureOutput {
	o.Error = err
	return o
}

type (
	DescribeSchemaOutput struct {
		Schema      *Schema
		Diagnostics hcl.Diagnostics // TODO: use local diagnostics type
	}
)
