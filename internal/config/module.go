package config

import (
	"fmt"

	"github.com/4rchr4y/godevkit/v3/maps"
	"github.com/4rchr4y/goray/internal/config/baseschema"
	version "github.com/hashicorp/go-version"
	"github.com/hashicorp/hcl/v2"
)

type Module struct {
	Source     string
	Input      *baseschema.Input
	Components map[string]*baseschema.Component
	Variables  map[string]*baseschema.Variable
	Includes   *baseschema.IncludeList
}

func NewModule(source string, files map[string]*baseschema.File) (mod *Module, v *version.Version, diagnostics hcl.Diagnostics) {
	mod = &Module{
		Source:     source,
		Components: make(map[string]*baseschema.Component),
		Variables:  make(map[string]*baseschema.Variable),
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
		case f.Input != nil && mod.Input != nil:
			diagnostics = diagnostics.Append(&hcl.Diagnostic{
				Severity: hcl.DiagWarning,
				Summary:  "Duplicated input block declaration",
				Detail:   fmt.Sprintf("A duplicate input block declaration was detected in file %s on line %d. This declaration will be ignored when building the configuration", fileName, f.Input.DeclRange.Start.Line),
			})
			continue
		case f.Input != nil:
			mod.Input = f.Input
		}

	}

	return mod, v, diagnostics
}
