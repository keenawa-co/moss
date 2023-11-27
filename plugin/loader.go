package plugin

import (
	"fmt"
	"plugin"

	"golang.org/x/tools/go/analysis"
)

func LoadPlugin(path string) (*analysis.Analyzer, error) {
	p, err := plugin.Open(path)
	if err != nil {
		return nil, err
	}

	symbol, err := p.Lookup("Analyzer")
	if err != nil {
		return nil, err
	}

	analyzer, ok := symbol.(*analysis.Analyzer)
	if !ok {
		return nil, fmt.Errorf("symbol 'Analyzer' %s is not a type *analysis.Analyzer", path)
	}

	return analyzer, nil
}
