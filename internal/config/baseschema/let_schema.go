package baseschema

import (
	"fmt"

	"github.com/4rchr4y/goray/internal/hcllang"
	"github.com/hashicorp/hcl/v2"
	"github.com/hashicorp/hcl/v2/ext/typeexpr"
	"github.com/hashicorp/hcl/v2/gohcl"
	"github.com/hashicorp/hcl/v2/hclsyntax"
	"github.com/zclconf/go-cty/cty"
	"github.com/zclconf/go-cty/cty/convert"
)

// Reserved for future expansion
var (
	letBlockReservedAttributeList = [...]string{
		"sensitive",
	}
	letBlockReservedBlockList = [...]string{
		"validation",
	}
)

var letBlockSchema = hcl.BodySchema{
	Attributes: hcllang.NewAttributeList(
		hcl.AttributeSchema{
			Name:     "type",
			Required: false,
		},
		hcl.AttributeSchema{
			Name:     "description",
			Required: false,
		},
		hcl.AttributeSchema{
			Name:     "default",
			Required: false,
		},
		hcl.AttributeSchema{
			Name:     "nullable",
			Required: false,
		},
	)(letBlockReservedAttributeList[:]...),
	Blocks: hcllang.NewBlockList()(letBlockReservedBlockList[:]...),
}

var letBlockDef = hcl.BlockHeaderSchema{
	Type: "let",
	LabelNames: []string{
		"name",
	},
}

type Let struct {
	Name           string
	ConstraintType cty.Type
	Type           cty.Type
	TypeDefaults   *typeexpr.Defaults
	Description    string
	Default        cty.Value
	Nullable       bool
	DeclRange      hcl.Range
}

func ValidateLetBlock(block *hcl.Block) (diagnostics hcl.Diagnostics) {
	if len(block.Labels) < 1 {
		diagnostics = append(diagnostics, &hcl.Diagnostic{
			Severity: hcl.DiagError,
			Summary:  "Variable name not found",
			Detail:   fmt.Sprintf("Variable name must be specified as the first block label, on line: %d", block.DefRange.Start.Line),
			Subject:  &block.DefRange,
		})
		return diagnostics
	}

	if !hclsyntax.ValidIdentifier(block.Labels[0]) {
		diagnostics = diagnostics.Append(&hcl.Diagnostic{
			Severity: hcl.DiagError,
			Summary:  "Invalid variable name",
			Detail:   fmt.Sprintf("Variable name is invalid. %s", hcllang.BadIdentDetail),
			Subject:  &block.LabelRanges[0],
		})

		return diagnostics
	}

	return diagnostics
}

func DecodeLetBlock(block *hcl.Block) (decodedBlock *Let, diagnostics hcl.Diagnostics) {
	content, diagnostics := block.Body.Content(&letBlockSchema)
	if diagnostics.HasErrors() {
		return nil, diagnostics
	}

	decodedBlock = &Let{
		Name:      block.Labels[0], // label presence was verified upon block detection
		DeclRange: block.DefRange,
	}

	if attr, exists := content.Attributes["type"]; exists {
		ty, defaults, diags := decodeVariableType(attr.Expr)
		diagnostics = append(diagnostics, diags...)

		decodedBlock.ConstraintType = ty
		decodedBlock.TypeDefaults = defaults
		decodedBlock.Type = ty.WithoutOptionalAttributesDeep()
	}

	if attr, exists := content.Attributes["description"]; exists {
		diags := gohcl.DecodeExpression(attr.Expr, nil, &decodedBlock.Description)
		diagnostics = append(diagnostics, diags...)
	}

	if attr, exists := content.Attributes["nullable"]; exists {
		diags := gohcl.DecodeExpression(attr.Expr, nil, &decodedBlock.Nullable)
		diagnostics = append(diagnostics, diags...)
	} else {
		decodedBlock.Nullable = true
	}

	if attr, exists := content.Attributes["default"]; exists {
		val, diags := attr.Expr.Value(nil)
		diagnostics = append(diagnostics, diags...)

		if decodedBlock.ConstraintType != cty.NilType {
			var err error

			// Should the type constraint include defaults, those
			// defaults should be applied to the variable's default
			// value prior to type conversion, except when the default
			// value is null. Null values are intentionally excluded
			// from this default application process, permitting
			// variables that can be null to possess a null default
			// value as a special case.
			if decodedBlock.TypeDefaults != nil && !val.IsNull() {
				val = decodedBlock.TypeDefaults.Apply(val)
			}

			val, err = convert.Convert(val, decodedBlock.ConstraintType)
			if err != nil {
				diagnostics = diagnostics.Append(&hcl.Diagnostic{
					Severity: hcl.DiagError,
					Summary:  "Invalid variable default value",
					Detail:   fmt.Sprintf("The default value does not adhere to the type constraint %s for the variable", err),
					Subject:  attr.Expr.Range().Ptr(),
				})

				val = cty.DynamicVal
			}
		}

		if !decodedBlock.Nullable && val.IsNull() {
			diagnostics = diagnostics.Append(&hcl.Diagnostic{
				Severity: hcl.DiagError,
				Summary:  "Invalid default value for variable",
				Detail:   "A null default value is not valid when nullable=false.",
				Subject:  attr.Expr.Range().Ptr(),
			})
		}

		decodedBlock.Default = val
	}

	return decodedBlock, diagnostics

}

func decodeVariableType(expr hcl.Expression) (ty cty.Type, defaults *typeexpr.Defaults, diagnostics hcl.Diagnostics) {
	switch hcl.ExprAsKeyword(expr) {
	case "list":
		return cty.List(cty.DynamicPseudoType), nil, nil
	case "map":
		return cty.Map(cty.DynamicPseudoType), nil, nil
	}

	return typeexpr.TypeConstraintWithDefaults(expr)
}
