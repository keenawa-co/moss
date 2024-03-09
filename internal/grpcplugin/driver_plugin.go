package grpcplugin

import (
	"github.com/4rchr4y/goray/internal/proto/protodriver"
	pluginHCL "github.com/hashicorp/go-plugin"
)

type DriverServeFn func() protodriver.DriverServer

type ServeDriverConf struct {
	GRPCServeFn DriverServeFn
}

func ServeDriver(conf *ServeDriverConf) {
	pluginHCL.Serve(&pluginHCL.ServeConfig{
		HandshakeConfig: Handshake,
		VersionedPlugins: map[int]pluginHCL.PluginSet{
			1: {
				"driver": &GRPCDriverPlugin{
					ServeFn: conf.GRPCServeFn,
				},
			},
		},
		GRPCServer: pluginHCL.DefaultGRPCServer,
	})
}
