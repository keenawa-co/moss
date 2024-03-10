package component

import "github.com/hashicorp/hcl/v2"

type Interface interface {
	Heartbeat() *HeartbeatOutput
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
	DescribeSchemaOutput struct {
		Schema      *Schema
		Diagnostics hcl.Diagnostics // TODO: use local diagnostics type
	}
)
