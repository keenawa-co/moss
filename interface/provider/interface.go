package provider

import "github.com/4rchr4y/goray/internal/schematica"

type Interface interface {
	DescribeSchema() *DescribeSchemaOutput
	ReadResource(*ReadResourceInput) *ReadResourceOutput
	Stop() error
	Shutdown() error
}

type (
	DescribeSchemaOutput struct {
		Schema *schematica.BlockSpec
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
