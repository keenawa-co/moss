package main

import (
	"fmt"
	"time"

	dummy_component "github.com/4rchr4y/goray/example/dummy-component"
	"github.com/4rchr4y/goray/internal/domain/grpcwrap"
	"github.com/4rchr4y/goray/internal/grpcplugin"
	"github.com/4rchr4y/goray/internal/proto/protocomponent"
)

func main() {
	go func() {
		time.Sleep(5 * time.Second)

		fmt.Println("HELLLOOOOO!")
	}()
	grpcplugin.ServeComponent(&grpcplugin.ServeComponentConf{
		GRPCServeFn: func() protocomponent.ComponentServer {
			return grpcwrap.ComponentWrapper(dummy_component.Component())
		},
	})
}
