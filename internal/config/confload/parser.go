package confload

import (
	"fmt"

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
