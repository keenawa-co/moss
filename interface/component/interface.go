package component

import (
	"github.com/4rchr4y/goray/diag"
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
		Status      Status
		Diagnostics diag.DiagnosticSet
	}
)

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
		Diagnostics diag.DiagnosticSet
	}
)
