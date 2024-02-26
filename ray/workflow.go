package ray

// Workflow file template schema

type WorkflowFileTemplate struct {
	Name string `hcl:"name,optional"`
}

type JobIncludeBlockDecl struct {
	Source  string            `hcl:"source,label"`
	Version string            `hcl:"source,label"`
	Comment string            `hcl:"comment,optional"`
	Use     map[string]string `hcl:"use"`
	Rules   []bool            `hcl:"rules,optional"`
}

type JobBlockDecl struct {
	Comment string                 `hcl:"comment,label"`
	Watch   []string               `hcl:"watch"`
	Steps   []*JobIncludeBlockDecl `hcl:"include,block"`
}

type WorkflowFileSchema struct {
	Name string          `hcl:"name,optional"`
	Jobs []*JobBlockDecl `hcl:"job,block"`
}

// Default workflow file schema
