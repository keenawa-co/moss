package provider

type Interface interface {
	DescribeSchema() *DescribeSchemaOutput
	ReadResource(*ReadResourceInput) *ReadResourceOutput
	Stop() error
	Shutdown() error
}

type (
	DescribeSchemaOutput struct {
		Schema *Schema
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
