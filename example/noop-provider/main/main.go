package main

import (
	noop_provider "github.com/4rchr4y/goray/example/noop-provider"
	"github.com/4rchr4y/goray/internal/domain/grpcwrap"
	"github.com/4rchr4y/goray/internal/plugin"
	"github.com/4rchr4y/goray/internal/proto/pluginproto"
)

func main() {
	plugin.Serve(&plugin.ServeConf{
		GRPCProviderFn: func() pluginproto.ProviderServer {
			return grpcwrap.Successor(noop_provider.Provider())
		},
	})
}
