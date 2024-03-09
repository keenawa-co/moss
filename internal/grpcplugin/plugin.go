package grpcplugin

import (
	"github.com/4rchr4y/goray/internal/proto/protodriver"
	pluginHCL "github.com/hashicorp/go-plugin"
)

const (
	ProtocolVersion = 1
)

var Handshake = pluginHCL.HandshakeConfig{
	ProtocolVersion:  ProtocolVersion,
	MagicCookieKey:   "RAY_MAGIC_COOKIE",
	MagicCookieValue: "dW5kZXJzdGFuZGluZyBob3cgbXkgY2FyIHdvcmtzIGhhcyBtYWRlIG1lIGEgYmV0dGVyIGRyaXZlci4=",
}

type ServeConf struct {
	GRPCDriverFn func() protodriver.DriverServer
}

func Serve(conf *ServeConf) {
	pluginHCL.Serve(&pluginHCL.ServeConfig{
		HandshakeConfig: Handshake,
		VersionedPlugins: map[int]pluginHCL.PluginSet{
			1: {
				"driver": &GRPCDriverPlugin{
					GRPCDriverFn: conf.GRPCDriverFn,
				},
			},
		},
		GRPCServer: pluginHCL.DefaultGRPCServer,
	})
}
