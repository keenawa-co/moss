package baseschema

import (
	"fmt"

	"github.com/4rchr4y/goray/internal/hclutl"
	"github.com/hashicorp/hcl/v2"
	"github.com/hashicorp/hcl/v2/gohcl"

	version "github.com/hashicorp/go-version"
)

// Reserved for future expansion
var (
	fileReservedAttributeList = [...]string{
		"edition",
	}
	fileReservedBlockList = [...]string{
		"package",
		"module",
		"import",
	}
)

var fileSchema = &hcl.BodySchema{
	Attributes: hclutl.NewAttributeList(
		// The version is not mandatory for every file, but it must
		// be specified in at least one of them. The version should
		// be declared a single time in any file deemed primary by
		// the user. The specified version will represent the
		// version of the entire module, and if the module is root,
		// it will denote the version of the entire configuration.
		hcl.AttributeSchema{
			Name:     "version",
			Required: false,
		},
	)(fileReservedAttributeList[:]...),
	Blocks: hclutl.NewBlockList(
		hcl.BlockHeaderSchema{
			Type:       "_",
			LabelNames: []string{},
		},
		hcl.BlockHeaderSchema{
			Type: "component",
			LabelNames: []string{
				"name",
			},
		},
		hcl.BlockHeaderSchema{
			Type: "variable",
			LabelNames: []string{
				"name",
			},
		},
	)(fileReservedBlockList[:]...),
}

type Version struct {
	DefRange hcl.Range
	Value    *version.Version
}

type File struct {
	_          [0]int
	Name       string
	Components map[string]*Component
	Variables  map[string]*Variable
	Body       hcl.Body
	Version    *Version
}

func DecodeFile(body hcl.Body) (file *File, diagnostics hcl.Diagnostics) {
	content, body, diagnostics := body.PartialContent(fileSchema)
	if diagnostics.HasErrors() {
		return nil, diagnostics
	}

	file = &File{
		Body:       body,
		Components: make(map[string]*Component),
		Variables:  make(map[string]*Variable),
	}

	if attr, exists := content.Attributes["version"]; exists {
		var (
			v   string
			err error
		)

		diags := gohcl.DecodeExpression(attr.Expr, nil, &v)
		diagnostics = append(diagnostics, diags...)
		if diagnostics.HasErrors() {
			return nil, diagnostics
		}

		file.Version = &Version{
			DefRange: attr.Range,
		}

		file.Version.Value, err = version.NewVersion(v)
		if err != nil {
			diagnostics = diagnostics.Append(&hcl.Diagnostic{
				Severity: hcl.DiagError,
				Summary:  "Invalid specified version",
				Detail:   fmt.Sprintf("Expected semantic version format MAJOR.MINOR.PATCH got %q", v),
			})
		}
	}

	for _, b := range content.Blocks {
		switch b.Type {
		case "component":
			c, diags := decodeComponentBlock(file, b)
			diagnostics = append(diagnostics, diags...)
			if diagnostics.HasErrors() {
				return nil, diagnostics
			}

			file.Components[c.Name] = c
			continue

		case "variable":
			v, diags := decodeVariableBlock(file, b)
			diagnostics = append(diagnostics, diags...)
			if diagnostics.HasErrors() {
				return nil, diagnostics
			}

			file.Variables[v.Name] = v
			continue
		}
	}

	return file, diagnostics
}

func decodeVariableBlock(file *File, block *hcl.Block) (v *Variable, diagnostics hcl.Diagnostics) {
	if len(block.Labels) < 1 {
		diagnostics = append(diagnostics, &hcl.Diagnostic{
			Severity: hcl.DiagError,
			Summary:  "Variable name not found",
			Detail:   fmt.Sprintf("Variable name must be specified as the first block label, on line: %d", block.DefRange.Start.Line),
			Subject:  &block.DefRange,
		})
		return nil, diagnostics
	}

	if _, exists := file.Variables[block.Labels[0]]; exists {
		diagnostics = append(diagnostics, &hcl.Diagnostic{
			Severity: hcl.DiagError,
			Summary:  "Duplicated variable",
			Detail:   fmt.Sprintf("A duplicate version declaration was detected in file %s on line %d. This declaration will be ignored when building the configuration", file.Name, block.DefRange.Start.Line),
			Subject:  &block.DefRange,
		})
		return nil, diagnostics
	}

	return DecodeVariableBlock(block)
}

func decodeComponentBlock(file *File, block *hcl.Block) (c *Component, diagnostics hcl.Diagnostics) {
	if len(block.Labels) < 1 {
		diagnostics = diagnostics.Append(&hcl.Diagnostic{
			Severity: hcl.DiagError,
			Summary:  "Component name not found",
			Detail:   fmt.Sprintf("Component name must be specified as the first block label, on line: %d", block.DefRange.Start.Line),
			Subject:  &block.DefRange,
		})
		return nil, diagnostics
	}

	if _, exists := file.Components[block.Labels[0]]; exists {
		diagnostics = diagnostics.Append(&hcl.Diagnostic{
			Severity: hcl.DiagError,
			Summary:  "Duplicated component",
			Detail:   fmt.Sprintf("A duplicate version declaration was detected in file %s on line %d. This declaration will be ignored when building the configuration", file.Name, block.DefRange.Start.Line),
			Subject:  &block.DefRange,
		})
		return nil, diagnostics
	}

	return DecodeComponentBlock(block)
}
