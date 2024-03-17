package confload

import (
	"fmt"
	"path/filepath"
	"strings"

	"github.com/4rchr4y/goray/internal/config"
	"github.com/4rchr4y/goray/internal/kernel/bis"
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

func (l *Loader) Load(dir string) (mod *config.Module, diagnostics hcl.Diagnostics) {
	infos, err := l.parser.fs.ReadDir(dir)
	if err != nil {
		diagnostics = append(diagnostics, &hcl.Diagnostic{
			Severity: hcl.DiagError,
			Summary:  "Failed to read template directory",
			Detail:   fmt.Sprintf("Template directory %s does not exist or is unreadable.", dir),
		})
		return nil, diagnostics
	}

	mod = &config.Module{
		Source:     dir,
		Components: make(map[string]*bis.ComponentBlock),
	}

	filePaths := make([]string, 0, len(infos))
	for i := range infos {
		if infos[i].IsDir() {
			continue
		}

		name := infos[i].Name()

		if strings.HasSuffix(name, ".ray") {
			filePaths = append(filePaths, filepath.Join(dir, name))
			continue
		}
	}

	files := make([]*bis.File, len(filePaths))
	for i := range filePaths {
		f, diags := l.parser.ParseHCLFile(filePaths[i])
		diagnostics = append(diagnostics, diags...)
		if diags.HasErrors() {
			return nil, diagnostics
		}

		files[i] = f
	}

	for _, file := range files {
		for name, component := range file.Components {
			mod.Components[name] = component
		}
	}

	return mod, diagnostics
}
