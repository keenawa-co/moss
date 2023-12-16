package openpolicy

import (
	"io"

	"github.com/open-policy-agent/opa/ast"
)

const (
	defaultTarget = "*"
	defaultQuery  = "data.goray"
	defaultOpaDir = "opa"
)

type PolicyGroup struct {
	Name     string
	Target   string
	Source   *ast.Module
	Requires map[string]*ast.Module
}

type Policy struct {
	Name    string
	Content []byte
}

type PolicyOptFn func(*Policy)

func NewPolicy(reader io.Reader, name string, options ...PolicyOptFn) (policy *Policy, err error) {
	policy = &Policy{
		Name: name,
	}

	policy.Content, err = io.ReadAll(reader)
	if err != nil {
		return nil, err
	}

	for i := 0; i < len(options); i++ {
		options[i](policy)
	}

	return policy, nil

}

func ParseModule(policy *Policy) (*ast.Module, error) {
	parsed, err := ast.ParseModule(policy.Name, string(policy.Content))
	if err != nil {
		// TODO add err msg
		return nil, err
	}

	return parsed, nil
}
