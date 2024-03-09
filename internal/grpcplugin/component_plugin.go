package grpcplugin

import (
	"github.com/4rchr4y/goray/internal/proto/protocomponent"
	pluginHCL "github.com/hashicorp/go-plugin"
)

type ComponentServeFn func() protocomponent.ComponentServer

type ServeComponentConf struct {
	GRPCServeFn ComponentServeFn
}

func ServeComponent(conf *ServeComponentConf) {
	pluginHCL.Serve(&pluginHCL.ServeConfig{
		HandshakeConfig: Handshake,
		VersionedPlugins: map[int]pluginHCL.PluginSet{
			1: {
				"component": &GRPCComponentPlugin{
					ServeFn: conf.GRPCServeFn,
				},
			},
		},
		GRPCServer: pluginHCL.DefaultGRPCServer,
	})
}
