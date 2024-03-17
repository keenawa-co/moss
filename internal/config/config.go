package config

import (
	version "github.com/hashicorp/go-version"
)

type Config struct {
	// Reference to the module that is directly invoking this specific module
	Parent *Config

	// Array of modules invoked within the configuration scope of this module
	Children map[string]*Config

	// A set of configuration settings specific to this module
	Module *Module

	// Specified configuration version
	Version *version.Version
}
