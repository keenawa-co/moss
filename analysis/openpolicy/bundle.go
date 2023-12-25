package openpolicy

import (
	"github.com/open-policy-agent/opa/ast"
)

type ModuleFile struct {
	Path   string
	Raw    []byte
	Parsed *ast.Module
}
