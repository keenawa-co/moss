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
	Components map[string]*baseschema.Component
	Variables  map[string]*baseschema.Variable
	Includes   *baseschema.IncludeList

	// The specified version will represent the version
	// of the entire module, and if the module is root, it
	// will denote the version of the entire configuration.
	Version *version.Version // TODO: Remove it
}

func NewModule(source string, files map[string]*baseschema.File) (mod *Module, diagnostics hcl.Diagnostics) {
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

		if f.Version != nil && mod.Version != nil {
			diagnostics = diagnostics.Append(&hcl.Diagnostic{
				Severity: hcl.DiagWarning,
				Summary:  "Duplication of versions",
				Detail:   fmt.Sprintf("A duplicate version declaration was detected in file %s on line %d. This declaration will be ignored when building the configuration", fileName, f.Version.DefRange.Start.Line),
			})
			continue
		}

		if f.Version != nil {
			mod.Version = f.Version.Value
		}
	}

	return mod, diagnostics
}
