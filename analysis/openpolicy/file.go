package openpolicy

import "github.com/open-policy-agent/opa/ast"

type File interface {
	FilePath() string
}

type RegoFile struct {
	Path   string
	Parsed *ast.Module
}

func (f *RegoFile) FilePath() string {
	return f.Path
}
