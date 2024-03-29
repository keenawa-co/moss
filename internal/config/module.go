package config

import (
	"fmt"

	"github.com/4rchr4y/godevkit/v3/maps"
	"github.com/4rchr4y/goray/internal/config/baseschema"
	version "github.com/hashicorp/go-version"
	"github.com/hashicorp/hcl/v2"
)

type Module struct {
	SourceDir  string
	Header     *baseschema.ModuleHeader
	Components map[string]*baseschema.Component
	Variables  map[string]*baseschema.Let
	Includes   *baseschema.IncludeList
}

func NewModule(source string, files map[string]*baseschema.File) (mod *Module, v *version.Version, diagnostics hcl.Diagnostics) {
	mod = &Module{
		SourceDir:  source,
		Components: make(map[string]*baseschema.Component),
		Variables:  make(map[string]*baseschema.Let),
		Includes: &baseschema.IncludeList{
			Modules: make(map[string]*baseschema.IncludeModule),
		},
	}

	for fileName, f := range files {
		mod.Components = maps.Merge(mod.Components, f.Components)
		mod.Variables = maps.Merge(mod.Variables, f.Variables)
		mod.Includes.Modules = maps.Merge(mod.Includes.Modules, f.Includes.Modules)

		switch {
		case f.Version != nil && v != nil:
			diagnostics = diagnostics.Append(&hcl.Diagnostic{
				Severity: hcl.DiagWarning,
				Summary:  "Duplication of version declaration",
				Detail:   fmt.Sprintf("A duplicate version declaration was detected in file %s on line %d. This declaration will be ignored when building the configuration", fileName, f.Version.DeclRange.Start.Line),
			})
			continue
		case f.Version != nil:
			v = f.Version.Value
		}

		switch {
		case f.ModuleHeader != nil && mod.Header != nil:
			diagnostics = diagnostics.Append(&hcl.Diagnostic{
				Severity: hcl.DiagWarning,
				Summary:  "Duplicated input block declaration",
				Detail:   fmt.Sprintf("A duplicate input block declaration was detected in file %s on line %d. This declaration will be ignored when building the configuration", fileName, f.ModuleHeader.DeclRange.Start.Line),
			})
			continue
		case f.ModuleHeader != nil:
			mod.Header = f.ModuleHeader
		}

	}

	return mod, v, diagnostics
}

type PropsMeta struct {
	AttributesSize uint
}

func (m *Module) PropsSchema() (*hcl.BodySchema, PropsMeta) {
	if m.Header == nil {
		return nil, PropsMeta{}
	}

	modPropsSchema := new(hcl.BodySchema)
	modPropsMeta := PropsMeta{}

	for varName, varDecl := range m.Header.Variables {
		fmt.Println(varDecl.Nullable)
		modPropsSchema.Attributes = append(modPropsSchema.Attributes, hcl.AttributeSchema{
			Name:     varName,
			Required: varDecl.Nullable,
		})
		modPropsMeta.AttributesSize++
	}

	return modPropsSchema, modPropsMeta
}
