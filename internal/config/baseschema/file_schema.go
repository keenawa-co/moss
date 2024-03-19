package baseschema

import (
	"fmt"

	"github.com/4rchr4y/goray/internal/hclutl"
	"github.com/4rchr4y/goray/internal/kernel/hcllang"
	"github.com/hashicorp/hcl/v2"
	"github.com/hashicorp/hcl/v2/gohcl"
	"github.com/hashicorp/hcl/v2/hclsyntax"

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
		hcl.BlockHeaderSchema{
			Type: "include_module",
			LabelNames: []string{
				"name",
			},
		},
		hcl.BlockHeaderSchema{
			Type:       "input",
			LabelNames: []string{},
		},
	)(fileReservedBlockList[:]...),
}

type Version struct {
	DeclRange hcl.Range
	Value     *version.Version
}

type IncludeList struct {
	Modules map[string]*IncludeModule
}

type File struct {
	_          [0]int
	Name       string
	Input      *Input // input block
	Components map[string]*Component
	Variables  map[string]*Variable
	Includes   *IncludeList
	Body       hcl.Body
	Version    *Version
	// TODO: FileType
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
		Includes: &IncludeList{
			Modules: make(map[string]*IncludeModule),
		},
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
			DeclRange: attr.Range,
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
			decoded, diags := decodeComponentBlock(file, b)
			diagnostics = append(diagnostics, diags...)
			if diagnostics.HasErrors() {
				return nil, diagnostics
			}

			file.Components[decoded.Name] = decoded
			continue

		case "variable":
			decoded, diags := decodeVariableBlock(file, b)
			diagnostics = append(diagnostics, diags...)
			if diagnostics.HasErrors() {
				return nil, diagnostics
			}

			file.Variables[decoded.Name] = decoded
			continue

		case "include_module":
			decoded, diags := decodeIncludeModuleBlock(file, b)
			diagnostics = append(diagnostics, diags...)
			if diagnostics.HasErrors() {
				return nil, diagnostics
			}

			file.Includes.Modules[decoded.Name] = decoded
			continue

		case "input":
			if file.Input != nil {
				diagnostics = diagnostics.Append(&hcl.Diagnostic{
					Severity: hcl.DiagError,
					Summary:  "Duplicated input block",
					Detail:   fmt.Sprintf("A duplicate input block declaration was detected in file %s on line %d. This declaration will be ignored when building the configuration", file.Name, b.DefRange.Start.Line),
					Subject:  &b.DefRange,
				})
				continue
			}

			decoded, diags := DecodeInputBlock(b)
			diagnostics = append(diagnostics, diags...)
			if diagnostics.HasErrors() {
				return nil, diagnostics
			}

			file.Input = decoded
			continue
		}
	}

	return file, diagnostics
}

// TODO: replace with validation func
func decodeIncludeModuleBlock(file *File, block *hcl.Block) (b *IncludeModule, diagnostics hcl.Diagnostics) {
	if len(block.Labels) < 1 {
		diagnostics = append(diagnostics, &hcl.Diagnostic{
			Severity: hcl.DiagError,
			Summary:  "Module name not found",
			Detail:   fmt.Sprintf("Module name must be specified as the first block label, on line: %d", block.DefRange.Start.Line),
			Subject:  &block.DefRange,
		})
		return nil, diagnostics
	}

	if _, exists := file.Variables[block.Labels[0]]; exists {
		diagnostics = append(diagnostics, &hcl.Diagnostic{
			Severity: hcl.DiagError,
			Summary:  "Duplicated module include",
			Detail:   fmt.Sprintf("Module %s in file %s on line %d was already included. This declaration will be ignored when building the configuration", block.Labels[0], file.Name, block.DefRange.Start.Line),
			Subject:  &block.DefRange,
		})
		return nil, diagnostics
	}

	if !hclsyntax.ValidIdentifier(block.Labels[0]) {
		diagnostics = diagnostics.Append(&hcl.Diagnostic{
			Severity: hcl.DiagError,
			Summary:  "Invalid module name",
			Detail:   fmt.Sprintf("Module name is invalid. %s", hcllang.BadIdentDetail),
			Subject:  &block.LabelRanges[0],
		})
		return nil, diagnostics
	}

	return DecodeIncludeModuleBlock(block)
}

// TODO: replace with validation func
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

// TODO: replace with validation func
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

	if !hclsyntax.ValidIdentifier(block.Labels[0]) {
		diagnostics = append(diagnostics, &hcl.Diagnostic{
			Severity: hcl.DiagError,
			Summary:  "Invalid component name",
			Detail:   fmt.Sprintf("Component name is invalid. %s", hcllang.BadIdentDetail),
			Subject:  &block.LabelRanges[0],
		})
		return nil, diagnostics
	}

	return DecodeComponentBlock(block)
}
