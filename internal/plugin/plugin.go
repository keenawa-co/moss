package plugin

import (
	pluginHCL "github.com/hashicorp/go-plugin"
)

var Handshake = pluginHCL.HandshakeConfig{
	ProtocolVersion:  1,
	MagicCookieKey:   "RAY_PLUGIN_MAGIC_COOKIE",
	MagicCookieValue: "hello",
}

type ServeConf struct {
	PluginMap map[string]pluginHCL.Plugin
}

func Serve(conf *ServeConf) {
	plugins := map[int]pluginHCL.PluginSet{
		1: conf.PluginMap,
	}

	pluginHCL.Serve(&pluginHCL.ServeConfig{
		HandshakeConfig:  Handshake,
		VersionedPlugins: plugins,
		GRPCServer:       pluginHCL.DefaultGRPCServer,
	})
}
