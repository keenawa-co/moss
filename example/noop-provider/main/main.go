package main

import (
	noop_provider "github.com/4rchr4y/goray/example/noop-provider"
	"github.com/4rchr4y/goray/internal/plugin"
	pluginHCL "github.com/hashicorp/go-plugin"
)

func main() {
	plugin.Serve(&plugin.ServeConf{
		PluginMap: map[string]pluginHCL.Plugin{
			"noop_provider": &noop_provider.NoopProviderPlugin{},
		},
	})
}
