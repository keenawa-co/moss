package rin

const (
	selfIndex     = 0
	templateIndex = 1
	fileNameIndex = 2
)

type ComponentDeclIdent []string

type ComponentDeclConf struct {
	Template TemplateIdent
	Path     string
	Name     string
}

// func NewComponentDecl(conf ComponentDeclConf) ComponentDeclIdent {
// 	builder := strings.Builder{}

// 	segments := []string{
// 		Prefix,
// 		Partition,

// 	}

// 	// builder.WriteString(Prefix)
// 	// builder.WriteString(Delimiter)
// 	// builder.WriteString(Partition)
// 	// builder.WriteString(Delimiter)

// }

// func (rin ComponentBlock) Template()
