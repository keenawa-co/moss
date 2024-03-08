package plugin

import (
	"github.com/4rchr4y/goray/internal/proto/pluginproto"
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
	GRPCProviderFn func() pluginproto.ProviderServer
}

func Serve(conf *ServeConf) {
	pluginHCL.Serve(&pluginHCL.ServeConfig{
		HandshakeConfig: Handshake,
		VersionedPlugins: map[int]pluginHCL.PluginSet{
			1: {
				"provider": &GRPCProviderPlugin{
					GRPCProvider: conf.GRPCProviderFn,
				},
			},
		},
		GRPCServer: pluginHCL.DefaultGRPCServer,
	})
}
