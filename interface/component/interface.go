package component

import (
	"github.com/4rchr4y/goray/diag"
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
		Diagnostics diag.DiagnosticSet
	}
)

type (
	DescribeSchemaOutput struct {
		Schema      *Schema
		Diagnostics hcl.Diagnostics // TODO: use local diagnostics type
	}
)
