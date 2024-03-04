package plugin

import (
	"github.com/hashicorp/go-plugin"
)

var Handshake = plugin.HandshakeConfig{
	ProtocolVersion:  1,
	MagicCookieKey:   "RAY_PLUGIN_MAGIC_COOKIE",
	MagicCookieValue: "HelloWorld",
}

func Serve() {
	plugin.Serve(&plugin.ServeConfig{
		HandshakeConfig:  Handshake,
		VersionedPlugins: map[int]plugin.PluginSet{},
		GRPCServer:       plugin.DefaultGRPCServer,
	})
}
