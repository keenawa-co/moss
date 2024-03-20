package baseschema

import (
	"fmt"

	"github.com/4rchr4y/goray/internal/hclutl"
	"github.com/hashicorp/hcl/v2"
)

// Reserved for future expansion
var (
	moduleHeaderReservedAttributeList = [...]string{}
	moduleHeaderReservedBlockList     = [...]string{}
)

var moduleHeaderBlockSchema = hcl.BodySchema{
	Attributes: hclutl.NewAttributeList()(moduleHeaderReservedAttributeList[:]...),
	Blocks: hclutl.NewBlockList(
		hcl.BlockHeaderSchema{
			Type: "variable",
			LabelNames: []string{
				"name",
			},
		},
	)(moduleHeaderReservedBlockList[:]...),
}

var moduleHeaderBlockDef = hcl.BlockHeaderSchema{
	Type:       "module_header",
	LabelNames: []string{},
}

type ModuleHeader struct {
	_         [0]int
	Variables map[string]*Variable
	DeclRange hcl.Range
}

func DecodeModuleHeaderBlock(block *hcl.Block) (decodedBlock *ModuleHeader, diagnostics hcl.Diagnostics) {
	content, diagnostics := block.Body.Content(&moduleHeaderBlockSchema)
	if diagnostics.HasErrors() {
		return nil, diagnostics
	}

	decodedBlock = &ModuleHeader{
		Variables: make(map[string]*Variable),
	}

	for _, b := range content.Blocks {
		switch b.Type {
		case "variable":
			diagnostics = append(diagnostics, ValidateVariableBlock(b)...)
			if diagnostics.HasErrors() {
				return nil, diagnostics
			}

			if _, exists := decodedBlock.Variables[b.Labels[0]]; exists {
				diagnostics = append(diagnostics, &hcl.Diagnostic{
					Severity: hcl.DiagError,
					Summary:  "Duplicated variable",
					Detail:   fmt.Sprintf("A duplicate variable declaration was detected in file %s on line %d. This declaration will be ignored when building the configuration", b.DefRange.Filename, b.DefRange.Start.Line),
					Subject:  &block.DefRange,
				})
			}

			decoded, diags := DecodeVariableBlock(b)
			diagnostics = append(diagnostics, diags...)
			if diagnostics.HasErrors() {
				return nil, diagnostics
			}

			decodedBlock.Variables[decoded.Name] = decoded
			continue
		}
	}

	return decodedBlock, diagnostics
}
