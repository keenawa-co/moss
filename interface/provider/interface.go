package provider

type Interface interface {
	GetSchema() *GetGetSchemaOutput
	ReadResource(*ReadResourceInput) *ReadResourceOutput
	DescribeResource(*DescribeResourceInput) *DescribeResourceOutput
	Stop() error
	Shutdown() error
}

type (
	GetGetSchemaOutput struct{}
)

type (
	ReadResourceInput  struct{}
	ReadResourceOutput struct{}
)

type (
	DescribeResourceInput  struct{}
	DescribeResourceOutput struct{}
)
