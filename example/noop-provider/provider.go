package noop_provider

import (
	"context"

	"github.com/4rchr4y/goray/internal/proto/pluginproto"
	pluginHCL "github.com/hashicorp/go-plugin"
	"google.golang.org/grpc"
)

type NoopProviderServer struct {
	pluginproto.UnimplementedProviderServer
}

func (s *NoopProviderServer) DescribeSchema(ctx context.Context, req *pluginproto.DescribeSchema_Request) (*pluginproto.DescribeSchema_Response, error) {
	return &pluginproto.DescribeSchema_Response{
		Provider: &pluginproto.Schema{
			Block: &pluginproto.Schema_Block{
				Description: "Hello, World!",
			},
		},
	}, nil
}

type NoopProviderPlugin struct {
	pluginHCL.Plugin
	Impl *NoopProviderServer
}

func (p *NoopProviderPlugin) GRPCServer(broker *pluginHCL.GRPCBroker, s *grpc.Server) error {
	pluginproto.RegisterProviderServer(s, p.Impl)
	return nil
}

func (p *NoopProviderPlugin) GRPCClient(ctx context.Context, broker *pluginHCL.GRPCBroker, c *grpc.ClientConn) (interface{}, error) {
	return pluginproto.NewProviderClient(c), nil
}
