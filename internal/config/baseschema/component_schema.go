package baseschema

import (
	"fmt"

	"github.com/4rchr4y/goray/internal/hclutl"
	"github.com/4rchr4y/goray/internal/kernel/hcllang"
	"github.com/hashicorp/hcl/v2"
	"github.com/hashicorp/hcl/v2/gohcl"
	"github.com/hashicorp/hcl/v2/hclsyntax"
)

// Reserved for future expansion
var (
	componentBlockReservedAttributeList = [...]string{}
	componentBlockReservedBlockList     = [...]string{}
)

var componentBlockSchema = hcl.BodySchema{
	Attributes: hclutl.NewAttributeList(
		hcl.AttributeSchema{
			Name:     "version",
			Required: false,
		},
	)(componentBlockReservedAttributeList[:]...),
	Blocks: hclutl.NewBlockList()(componentBlockReservedBlockList[:]...),
}

var componentBlockDef = hcl.BlockHeaderSchema{
	Type: "component",
	LabelNames: []string{
		"name",
	},
}

type Component struct {
	_       [0]int
	Name    string
	Version string
	Config  hcl.Body
}

func ValidateComponentBlock(block *hcl.Block) (diagnostics hcl.Diagnostics) {
	if len(block.Labels) < 1 {
		diagnostics = diagnostics.Append(&hcl.Diagnostic{
			Severity: hcl.DiagError,
			Summary:  "Component name not found",
			Detail:   fmt.Sprintf("Component name must be specified as the first block label, on line: %d", block.DefRange.Start.Line),
			Subject:  &block.DefRange,
		})
		return diagnostics
	}

	if !hclsyntax.ValidIdentifier(block.Labels[0]) {
		diagnostics = append(diagnostics, &hcl.Diagnostic{
			Severity: hcl.DiagError,
			Summary:  "Invalid component name",
			Detail:   fmt.Sprintf("Component name is invalid. %s", hcllang.BadIdentDetail),
			Subject:  &block.LabelRanges[0],
		})
		return diagnostics
	}

	return diagnostics
}

func DecodeComponentBlock(block *hcl.Block) (decodedBlock *Component, diagnostics hcl.Diagnostics) {
	content, body, diagnostics := block.Body.PartialContent(&componentBlockSchema)
	if diagnostics.HasErrors() {
		return nil, diagnostics
	}

	decodedBlock = &Component{
		Name:   block.Labels[0], // label presence was verified upon block detection
		Config: body,
	}

	if attr, exists := content.Attributes["version"]; exists {
		diags := gohcl.DecodeExpression(attr.Expr, nil, &decodedBlock.Version)
		diagnostics = append(diagnostics, diags...)
		if diagnostics.HasErrors() {
			return nil, diagnostics
		}
	}

	return decodedBlock, diagnostics
}
