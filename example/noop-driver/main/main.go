package main

import (
	noop_driver "github.com/4rchr4y/goray/example/noop-driver"
	"github.com/4rchr4y/goray/internal/grpcplugin"
	"github.com/4rchr4y/goray/internal/grpcwrap"
	"github.com/4rchr4y/goray/internal/proto/protodriver"
)

func main() {
	grpcplugin.ServeDriver(&grpcplugin.ServeDriverConf{
		GRPCServeFn: func() protodriver.DriverServer {
			return grpcwrap.DriverWrapper(noop_driver.Driver())
		},
	})
}
