package config

import (
	"fmt"

	"github.com/4rchr4y/goray/internal/kernel/bis"
	"github.com/hashicorp/hcl/v2"
)

type Module struct {
	Source     string
	Components map[string]*bis.ComponentBlock
}

func NewModule(source string, files map[string]*bis.File) (mod *Module, diagnostics hcl.Diagnostics) {
	mod = &Module{
		Source:     source,
		Components: make(map[string]*bis.ComponentBlock),
	}

	for _, f := range files {
		for componentName, c := range f.Components {
			addr := fmt.Sprintf("component:%s", componentName) // TODO: replace with RIN
			mod.Components[addr] = c
		}
	}

	return mod, diagnostics
}
