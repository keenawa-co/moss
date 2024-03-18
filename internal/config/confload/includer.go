package confload

import (
	"github.com/4rchr4y/goray/internal/config"
	"github.com/hashicorp/hcl/v2"
)

type includer struct {
	parser *Parser
}

func (i *includer) IncludeModule(source string) (mod *config.Module, diagnostics hcl.Diagnostics) {
	return i.parser.ParseModDir(source)
}
