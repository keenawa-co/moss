package openpolicy

import (
	"github.com/open-policy-agent/opa/ast"
)

const (
	defaultTarget = "*"
	defaultQuery  = "data.goray"
)

type Policy struct {
	Name    string
	Target  string
	Source  *ast.Module
	Vendors map[string]*ast.Module
}

type QueryPass struct {
	Query    string
	Compiler *ast.Compiler
	Target   interface{}
}

type Group struct {
	Target   string
	Modules  map[string]*ast.Module
	Requires []*Group
	Query    func(pass *QueryPass)
}

func ParseModule(name string, content []byte) (*ast.Module, error) {
	parsed, err := ast.ParseModule(name, string(content))
	if err != nil {
		// TODO add err msg
		return nil, err
	}

	return parsed, nil
}
