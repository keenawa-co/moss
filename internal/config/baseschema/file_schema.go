package baseschema

import (
	"fmt"

	"github.com/4rchr4y/goray/internal/hcllang"
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

var fileSchema = hcl.BodySchema{
	Attributes: hcllang.NewAttributeList(
		hcl.AttributeSchema{
			Name:     "version",
			Required: false,
		},
	)(fileReservedAttributeList[:]...),
	Blocks: hcllang.NewBlockList(
		hcl.BlockHeaderSchema{
			Type:       "_",
			LabelNames: []string{},
		},
		componentBlockDef,
		letBlockDef,
		includeModuleBlockDef,
		moduleHeaderBlockDef,
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
	_            [0]int
	Name         string
	ModuleHeader *ModuleHeader
	Components   map[string]*Component
	Variables    map[string]*Let
	Includes     *IncludeList
	Body         hcl.Body
	Version      *Version
}

func DecodeFile(body hcl.Body) (decodedBlock *File, diagnostics hcl.Diagnostics) {
	content, body, diagnostics := body.PartialContent(&fileSchema)
	if diagnostics.HasErrors() {
		return nil, diagnostics
	}

	decodedBlock = &File{
		Body:       body,
		Components: make(map[string]*Component),
		Variables:  make(map[string]*Let),
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

		decodedBlock.Version = &Version{
			DeclRange: attr.Range,
		}

		decodedBlock.Version.Value, err = version.NewVersion(v)
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
		case componentBlockDef.Type:
			diagnostics = append(diagnostics, ValidateComponentBlock(b)...)
			if diagnostics.HasErrors() {
				return nil, diagnostics
			}

			if _, exists := decodedBlock.Components[b.Labels[0]]; exists {
				diagnostics = diagnostics.Append(&hcl.Diagnostic{
					Severity: hcl.DiagError,
					Summary:  "Duplicated component",
					Detail:   fmt.Sprintf("A duplicate version declaration was detected in file %s on line %d. This declaration will be ignored when building the configuration", decodedBlock.Name, b.DefRange.Start.Line),
					Subject:  &b.DefRange,
				})
				return nil, diagnostics
			}

			decoded, diags := DecodeComponentBlock(b)
			diagnostics = append(diagnostics, diags...)
			if diagnostics.HasErrors() {
				return nil, diagnostics
			}

			decodedBlock.Components[decoded.Name] = decoded
			continue

		case letBlockDef.Type:
			diagnostics = append(diagnostics, ValidateLetBlock(b)...)
			if diagnostics.HasErrors() {
				return nil, diagnostics
			}

			if _, exists := decodedBlock.Variables[b.Labels[0]]; exists {
				diagnostics = append(diagnostics, &hcl.Diagnostic{
					Severity: hcl.DiagError,
					Summary:  "Duplicated variable",
					Detail:   fmt.Sprintf("A duplicate variable declaration was detected in file %s on line %d. This declaration will be ignored when building the configuration", decodedBlock.Name, b.DefRange.Start.Line),
					Subject:  &b.DefRange,
				})
			}

			decoded, diags := DecodeLetBlock(b)
			diagnostics = append(diagnostics, diags...)
			if diagnostics.HasErrors() {
				return nil, diagnostics
			}

			decodedBlock.Variables[decoded.Name] = decoded
			continue

		case includeModuleBlockDef.Type:
			diagnostics = append(diagnostics, ValidateIncludeModuleBlock(b)...)
			if diagnostics.HasErrors() {
				return nil, diagnostics
			}

			if _, exists := decodedBlock.Variables[b.Labels[0]]; exists {
				diagnostics = append(diagnostics, &hcl.Diagnostic{
					Severity: hcl.DiagError,
					Summary:  "Duplicated module include",
					Detail:   fmt.Sprintf("Module %s in file %s on line %d was already included. This declaration will be ignored when building the configuration", b.Labels[0], decodedBlock.Name, b.DefRange.Start.Line),
					Subject:  &b.DefRange,
				})
				return nil, diagnostics
			}

			decoded, diags := DecodeIncludeModuleBlock(b)
			diagnostics = append(diagnostics, diags...)
			if diagnostics.HasErrors() {
				return nil, diagnostics
			}

			decodedBlock.Includes.Modules[decoded.Name] = decoded
			continue

		case moduleHeaderBlockDef.Type:
			if decodedBlock.ModuleHeader != nil {
				diagnostics = diagnostics.Append(&hcl.Diagnostic{
					Severity: hcl.DiagError,
					Summary:  "Duplicated module_header block",
					Detail:   fmt.Sprintf("A duplicate module_header block declaration was detected in file %s on line %d. This declaration will be ignored when building the configuration", decodedBlock.Name, b.DefRange.Start.Line),
					Subject:  &b.DefRange,
				})
				continue
			}

			decoded, diags := DecodeModuleHeaderBlock(b)
			diagnostics = append(diagnostics, diags...)
			if diagnostics.HasErrors() {
				return nil, diagnostics
			}

			decodedBlock.ModuleHeader = decoded
			continue
		}
	}

	return decodedBlock, diagnostics
}
