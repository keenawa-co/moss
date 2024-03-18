package config

import (
	version "github.com/hashicorp/go-version"
	"github.com/hashicorp/hcl/v2"
)

type Config struct {
	Root *Config
	// Reference to the module that is directly invoking this specific module
	Parent *Config

	// Array of modules invoked within the configuration scope of this module
	Children map[string]*Config

	// A set of configuration settings specific to this module
	Module *Module

	// Specified configuration version
	Version *version.Version
}

type IncludeModuleInput struct {
	_      [0]int
	Source string
	Parent *Config
}

type Includer interface {
	IncludeModule(source string) (*Module, hcl.Diagnostics)
}

// type ModuleIncluderFn func(input IncludeModuleInput) (*Module, hcl.Diagnostics)

// func (include ModuleIncluderFn) IncludeModule(input IncludeModuleInput) (*Module, hcl.Diagnostics) {
// 	return include(input)
// }

func BuildConfig(root *Module, includer Includer) (conf *Config, diagnostics hcl.Diagnostics) {
	conf = &Config{
		// Root:   conf,
		Module: root,
	}

	children, diags := buildChildren(conf, includer)
	diagnostics = append(diagnostics, diags...)
	conf.Children = children

	return conf, diagnostics

}

func buildChildren(parent *Config, includer Includer) (children map[string]*Config, diagnostics hcl.Diagnostics) {
	children = make(map[string]*Config)

	for name, includeModule := range parent.Module.Includes.Modules {
		child, diags := buildChild(parent.Root, includer, &IncludeModuleInput{
			Source: includeModule.Source,
			Parent: parent,
		})
		diagnostics = append(diagnostics, diags...)
		if child == nil {
			return nil, diagnostics
		}

		children[name] = child
	}

	return children, diagnostics
}

func buildChild(root *Config, includer Includer, input *IncludeModuleInput) (child *Config, diagnostics hcl.Diagnostics) {
	mod, diags := includer.IncludeModule(input.Source)
	diagnostics = append(diagnostics, diags...)
	if mod == nil {
		return nil, diagnostics
	}

	child = &Config{
		Root:    root,
		Parent:  input.Parent,
		Module:  mod,
		Version: mod.Version,
	}

	child.Children, diags = buildChildren(child, includer)
	diagnostics = append(diagnostics, diags...)

	return child, diagnostics
}
