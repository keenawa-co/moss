package baseschema

import (
	"fmt"

	"github.com/4rchr4y/goray/internal/hclutl"
	"github.com/hashicorp/hcl/v2"
	"github.com/hashicorp/hcl/v2/ext/typeexpr"
	"github.com/hashicorp/hcl/v2/gohcl"
	"github.com/zclconf/go-cty/cty"
	"github.com/zclconf/go-cty/cty/convert"
)

// Reserved for future expansion
var (
	variableBlockReservedAttributeList = [...]string{
		"sensitive",
	}
	variableBlockReservedBlockList = [...]string{
		"validation",
	}
)

var variableBlockSchema = &hcl.BodySchema{
	Attributes: hclutl.NewAttributeList(
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
	)(variableBlockReservedAttributeList[:]...),
	Blocks: hclutl.NewBlockList()(variableBlockReservedBlockList[:]...),
}

type Variable struct {
	Name           string
	ConstraintType cty.Type
	Type           cty.Type
	TypeDefaults   *typeexpr.Defaults
	Description    string
	Default        cty.Value
	Nullable       bool
	DeclRange      hcl.Range
}

func DecodeVariableBlock(block *hcl.Block) (variable *Variable, diagnostics hcl.Diagnostics) {
	content, diagnostics := block.Body.Content(variableBlockSchema)
	if diagnostics.HasErrors() {
		return nil, diagnostics
	}

	// if IsReserved(block.Labels[0]) {
	// 	diagnostics = diagnostics.Append(&hcl.Diagnostic{
	// 		Severity: hcl.DiagError,
	// 		Summary:  "Forbidden variable name used",
	// 		Detail:   fmt.Sprintf("Variable name %q is a reserved keyword.", block.Labels[0]),
	// 		Subject:  &block.LabelRanges[0],
	// 	})

	// 	return nil, diagnostics
	// }

	variable = &Variable{
		Name:      block.Labels[0], // label presence was verified upon block detection
		DeclRange: block.DefRange,
	}

	if attr, exists := content.Attributes["type"]; exists {
		ty, defaults, diags := decodeVariableType(attr.Expr)
		diagnostics = append(diagnostics, diags...)

		variable.ConstraintType = ty
		variable.TypeDefaults = defaults
		variable.Type = ty.WithoutOptionalAttributesDeep()
	}

	if attr, exists := content.Attributes["description"]; exists {
		diags := gohcl.DecodeExpression(attr.Expr, nil, &variable.Description)
		diagnostics = append(diagnostics, diags...)
	}

	if attr, exists := content.Attributes["nullable"]; exists {
		diags := gohcl.DecodeExpression(attr.Expr, nil, &variable.Nullable)
		diagnostics = append(diagnostics, diags...)
	} else {
		variable.Nullable = true
	}

	if attr, exists := content.Attributes["default"]; exists {
		val, diags := attr.Expr.Value(nil)
		diagnostics = append(diagnostics, diags...)

		if variable.ConstraintType != cty.NilType {
			var err error

			// Should the type constraint include defaults, those
			// defaults should be applied to the variable's default
			// value prior to type conversion, except when the default
			// value is null. Null values are intentionally excluded
			// from this default application process, permitting
			// variables that can be null to possess a null default
			// value as a special case.
			if variable.TypeDefaults != nil && !val.IsNull() {
				val = variable.TypeDefaults.Apply(val)
			}
			val, err = convert.Convert(val, variable.ConstraintType)
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

		if !variable.Nullable && val.IsNull() {
			diagnostics = diagnostics.Append(&hcl.Diagnostic{
				Severity: hcl.DiagError,
				Summary:  "Invalid default value for variable",
				Detail:   "A null default value is not valid when nullable=false.",
				Subject:  attr.Expr.Range().Ptr(),
			})
		}

		variable.Default = val
	}

	return variable, diagnostics

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
