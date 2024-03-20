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
	includeModuleBlockReservedAttributeList = [...]string{}
	includeModuleBlockReservedBlockList     = [...]string{
		"lifecycle",
	}
)

var includeModuleBlockSchema = hcl.BodySchema{
	Attributes: hclutl.NewAttributeList(
		hcl.AttributeSchema{
			Name:     "source",
			Required: true,
		},
	)(includeModuleBlockReservedAttributeList[:]...),
	Blocks: hclutl.NewBlockList()(includeModuleBlockReservedBlockList[:]...),
}

var includeModuleBlockDef = hcl.BlockHeaderSchema{
	Type: "include_module",
	LabelNames: []string{
		"name",
	},
}

type IncludeModule struct {
	Name      string
	Source    string
	Config    hcl.Body
	DeclRange hcl.Range
}

func ValidateIncludeModuleBlock(block *hcl.Block) (diagnostics hcl.Diagnostics) {
	if len(block.Labels) < 1 {
		diagnostics = append(diagnostics, &hcl.Diagnostic{
			Severity: hcl.DiagError,
			Summary:  "Module name not found",
			Detail:   fmt.Sprintf("Module name must be specified as the first block label, on line: %d", block.DefRange.Start.Line),
			Subject:  &block.DefRange,
		})
		return diagnostics
	}

	if !hclsyntax.ValidIdentifier(block.Labels[0]) {
		diagnostics = diagnostics.Append(&hcl.Diagnostic{
			Severity: hcl.DiagError,
			Summary:  "Invalid module name",
			Detail:   fmt.Sprintf("Module name is invalid. %s", hcllang.BadIdentDetail),
			Subject:  &block.LabelRanges[0],
		})
		return diagnostics
	}

	return diagnostics
}

func DecodeIncludeModuleBlock(block *hcl.Block) (b *IncludeModule, diagnostics hcl.Diagnostics) {
	content, body, diagnostics := block.Body.PartialContent(&includeModuleBlockSchema)
	if diagnostics.HasErrors() {
		return nil, diagnostics
	}

	b = &IncludeModule{
		Name:      block.Labels[0], // label presence was verified upon block detection
		Config:    body,
		DeclRange: block.DefRange,
	}

	if attr, exists := content.Attributes["source"]; exists {
		diags := gohcl.DecodeExpression(attr.Expr, nil, &b.Source)
		diagnostics = append(diagnostics, diags...)
		if diagnostics.HasErrors() {
			return nil, diagnostics
		}
	}

	return b, diagnostics
}
