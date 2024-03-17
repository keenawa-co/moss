package confload

import (
	"fmt"
	"path/filepath"
	"strings"

	"github.com/4rchr4y/goray/internal/config"
	"github.com/4rchr4y/goray/internal/config/baseschema"
	"github.com/hashicorp/hcl/v2"
	"github.com/hashicorp/hcl/v2/hclparse"
	"github.com/spf13/afero"
)

type Parser struct {
	fs   afero.Afero
	hclp *hclparse.Parser
}

func NewParser(fs afero.Fs) *Parser {
	return &Parser{
		fs:   afero.Afero{Fs: fs},
		hclp: hclparse.NewParser(),
	}
}

func (p *Parser) ParseConfDir(dir string) (mod *config.Module, diagnostics hcl.Diagnostics) {
	infos, err := p.fs.ReadDir(dir)
	if err != nil {
		diagnostics = append(diagnostics, &hcl.Diagnostic{
			Severity: hcl.DiagError,
			Summary:  "Failed to read template directory",
			Detail:   fmt.Sprintf("Template directory %s does not exist or is unreadable.", dir),
		})
		return nil, diagnostics
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

	files := make(map[string]*baseschema.File, len(filePaths))
	for i := range filePaths {
		f, diags := p.ParseHCLFile(filePaths[i])
		diagnostics = append(diagnostics, diags...)
		if diags.HasErrors() {
			return nil, diagnostics
		}

		files[filePaths[i]] = f
	}

	return config.NewModule(dir, files)
}

func (p *Parser) ParseHCLFile(path string) (file *baseschema.File, diagnostics hcl.Diagnostics) {
	content, err := p.fs.ReadFile(path)
	if err != nil {
		return nil, diagnostics.Append(&hcl.Diagnostic{
			Severity: hcl.DiagError,
			Summary:  "Failed to read file",
			Detail:   fmt.Sprintf("The contents of file %q could not be read.", path),
		})
	}

	parsedHCLFile, diagnostics := p.hclp.ParseHCL(content, path)
	if diagnostics.HasErrors() {
		return nil, diagnostics
	}

	file, diags := baseschema.DecodeFile(parsedHCLFile.Body)
	diagnostics = append(diagnostics, diags...)
	if diagnostics.HasErrors() {
		return nil, diagnostics
	}

	return file, diagnostics
}
