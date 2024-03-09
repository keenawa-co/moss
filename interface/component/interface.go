package component

import "github.com/hashicorp/hcl/v2"

type Interface interface {
	DescribeSchema() *DescribeSchemaOutput
	Stop() error
	Shutdown() error
}

type (
	DescribeSchemaOutput struct {
		Schema      *Schema
		Diagnostics hcl.Diagnostics // TODO: use local diagnostics type
	}
)
