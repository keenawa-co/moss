package regom

import "github.com/open-policy-agent/opa/ast"

type RegoFile struct {
	Path   string
	Raw    []byte
	Parsed *ast.Module
}

type Bundle struct {
	Name  string
	Files []*RegoFile
}
