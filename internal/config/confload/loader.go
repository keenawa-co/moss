package confload

import (
	"github.com/4rchr4y/goray/internal/config"

	"github.com/hashicorp/hcl/v2"
)

// TODO: add list of supported file extensions
type Loader struct {
	parser *Parser
}

func NewLoader(p *Parser) *Loader {
	return &Loader{
		parser: p,
	}
}

func (l *Loader) LoadConf(dir string) (conf *config.Config, diagnostics hcl.Diagnostics) {
	rootMod, diagnostics := l.parser.ParseConfDir(dir)
	if diagnostics.HasErrors() {
		return nil, diagnostics
	}

	return &config.Config{
		Parent:   nil,
		Children: nil,
		Module:   rootMod,
		Version:  rootMod.Version,
	}, diagnostics
}
