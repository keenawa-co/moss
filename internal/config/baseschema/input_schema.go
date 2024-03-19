package baseschema

import (
	"fmt"

	"github.com/4rchr4y/goray/internal/hclutl"
	"github.com/4rchr4y/goray/internal/kernel/hcllang"
	"github.com/hashicorp/hcl/v2"
	"github.com/hashicorp/hcl/v2/hclsyntax"
)

// Reserved for future expansion
var (
	inputBlockReservedAttributeList = [...]string{}
	inputBlockReservedBlockList     = [...]string{}
)

var inputBlockSchema = &hcl.BodySchema{
	Attributes: hclutl.NewAttributeList()(inputBlockReservedAttributeList[:]...),
	Blocks: hclutl.NewBlockList(
		hcl.BlockHeaderSchema{
			Type: "variable",
			LabelNames: []string{
				"name",
			},
		},
	)(inputBlockReservedBlockList[:]...),
}

type Input struct {
	_         [0]int
	Variables map[string]*Variable
	DeclRange hcl.Range
}

// TODO: use everywhere decodedBlock name
func DecodeInputBlock(block *hcl.Block) (decodedBlock *Input, diagnostics hcl.Diagnostics) {
	content, diagnostics := block.Body.Content(inputBlockSchema)
	if diagnostics.HasErrors() {
		return nil, diagnostics
	}

	decodedBlock = &Input{
		Variables: make(map[string]*Variable),
	}

	for _, b := range content.Blocks {
		switch b.Type {

		case "variable":
			decoded, diags := decodeVariableBlock2(decodedBlock, b)
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

// FIXME: use validation function instead of creation this garbage
func decodeVariableBlock2(input *Input, block *hcl.Block) (v *Variable, diagnostics hcl.Diagnostics) {
	if len(block.Labels) < 1 {
		diagnostics = append(diagnostics, &hcl.Diagnostic{
			Severity: hcl.DiagError,
			Summary:  "Variable name not found",
			Detail:   fmt.Sprintf("Variable name must be specified as the first block label, on line: %d", block.DefRange.Start.Line),
			Subject:  &block.DefRange,
		})
		return nil, diagnostics
	}

	if _, exists := input.Variables[block.Labels[0]]; exists {
		diagnostics = append(diagnostics, &hcl.Diagnostic{
			Severity: hcl.DiagError,
			Summary:  "Duplicated variable",
			// TODO: Detail:   fmt.Sprintf("A duplicate version declaration was detected in file %s on line %d. This declaration will be ignored when building the configuration", file.Name, block.DefRange.Start.Line),
			Subject: &block.DefRange,
		})
		return nil, diagnostics
	}

	if !hclsyntax.ValidIdentifier(block.Labels[0]) {
		diagnostics = diagnostics.Append(&hcl.Diagnostic{
			Severity: hcl.DiagError,
			Summary:  "Invalid variable name",
			Detail:   fmt.Sprintf("Variable name is invalid. %s", hcllang.BadIdentDetail),
			Subject:  &block.LabelRanges[0],
		})

		return nil, diagnostics
	}

	return DecodeVariableBlock(block)
}
