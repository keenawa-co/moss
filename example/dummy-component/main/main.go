package main

import (
	dummy_component "github.com/4rchr4y/goray/example/dummy-component"
	"github.com/4rchr4y/goray/internal/domain/grpcwrap"
	"github.com/4rchr4y/goray/internal/grpcplugin"
	"github.com/4rchr4y/goray/internal/proto/protocomponent"
)

func main() {
	grpcplugin.ServeComponent(&grpcplugin.ServeComponentConf{
		GRPCServeFn: func() protocomponent.ComponentServer {
			return grpcwrap.ComponentWrapper(dummy_component.Component())
		},
	})
}
