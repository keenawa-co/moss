package provider

import "github.com/hashicorp/hcl/v2"

type Interface interface {
	DescribeSchema() *DescribeSchemaOutput
	ReadResource(*ReadResourceInput) *ReadResourceOutput
	Stop() error
	Shutdown() error
}

type (
	DescribeSchemaOutput struct {
		Schema      *Schema
		Diagnostics hcl.Diagnostics
	}
)

type (
	ReadResourceInput  struct{}
	ReadResourceOutput struct{}
)

type (
	DescribeResourceInput  struct{}
	DescribeResourceOutput struct{}
)
