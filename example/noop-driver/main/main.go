package main

import (
	noop_driver "github.com/4rchr4y/goray/example/noop-driver"
	"github.com/4rchr4y/goray/internal/domain/grpcwrap"
	"github.com/4rchr4y/goray/internal/grpcplugin"
	"github.com/4rchr4y/goray/internal/proto/protodriver"
)

func main() {
	grpcplugin.Serve(&grpcplugin.ServeConf{
		GRPCDriverFn: func() protodriver.DriverServer {
			return grpcwrap.Successor(noop_driver.Driver())
		},
	})
}
