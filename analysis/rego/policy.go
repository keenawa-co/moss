package openpolicy

import (
	"io"

	"github.com/open-policy-agent/opa/ast"
	"github.com/open-policy-agent/opa/rego"
)

const (
	defaultTarget = "*"
	defaultQuery  = "data.goray"
)

type PolicyGroup struct {
	Name   string
	List   []*rego.PreparedEvalQuery
	Target string
}

type Policy struct {
	_        [0]int
	Name     string
	Content  []byte
	Target   string
	Query    string
	Requires []*Policy
}

type PolicyOptFn func(*Policy)

func NewPolicy(reader io.Reader, name string, options ...PolicyOptFn) (policy *Policy, err error) {
	policy = &Policy{
		Name:   name,
		Target: defaultTarget,
		Query:  defaultQuery,
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

func WithTarget(target string) PolicyOptFn {
	return func(p *Policy) {
		p.Target = target
	}
}

func ParseModule(policy *Policy) (*ast.Module, error) {
	parsed, err := ast.ParseModule(policy.Name, string(policy.Content))
	if err != nil {
		// TODO add err msg
		return nil, err
	}

	return parsed, nil
}
