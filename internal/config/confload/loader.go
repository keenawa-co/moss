package confload

import (
	"github.com/4rchr4y/goray/internal/config"

	"github.com/hashicorp/hcl/v2"
)

type Loader struct {
	parser *Parser
}

func NewLoader(p *Parser) *Loader {
	return &Loader{
		parser: p,
	}
}

func (l *Loader) LoadConf(dir string) (conf *config.Config, diagnostics hcl.Diagnostics) {
	rootMod, diagnostics := l.parser.ParseModDir(dir)
	if diagnostics.HasErrors() {
		return nil, diagnostics
	}

	return config.BuildConfig(rootMod, &includer{
		parser: l.parser,
	})
}
