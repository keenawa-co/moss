package component

import (
	"github.com/4rchr4y/goray/internal/schematica"
	"github.com/hashicorp/hcl/v2"
)

type Interface interface {
	Heartbeat() *HeartbeatOutput
	Configure(*ConfigureInput) (*ConfigureOutput, error)
	DescribeSchema() *DescribeSchemaOutput
	Stop() error
	Shutdown() error
}

type (
	HeartbeatOutput struct {
		Status string
	}
)

type (
	ConfigureInput struct {
		Name    string
		Version string
		Schema  *schematica.Block
	}

	ConfigureOutput struct{}
)

type (
	DescribeSchemaOutput struct {
		Schema      *Schema
		Diagnostics hcl.Diagnostics // TODO: use local diagnostics type
	}
)
