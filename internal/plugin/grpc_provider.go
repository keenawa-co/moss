package plugin

import (
	"context"

	"github.com/4rchr4y/goray/interface/provider"
	"github.com/4rchr4y/goray/internal/proto/pluginproto"
	"google.golang.org/grpc"

	pluginHCL "github.com/hashicorp/go-plugin"
)

type GRPCProviderPlugin struct {
	pluginHCL.Plugin
	GRPCProvider func() pluginproto.ProviderServer
}

func (p *GRPCProviderPlugin) GRPCClient(ctx context.Context, broker *pluginHCL.GRPCBroker, c *grpc.ClientConn) (interface{}, error) {
	return &GRPCProvider{
		client: pluginproto.NewProviderClient(c),
		ctx:    ctx,
	}, nil
}

func (p *GRPCProviderPlugin) GRPCServer(broker *pluginHCL.GRPCBroker, s *grpc.Server) error {
	pluginproto.RegisterProviderServer(s, p.GRPCProvider())
	return nil
}

type GRPCProvider struct {
	PluginClient *pluginHCL.Client
	ctx          context.Context
	client       pluginproto.ProviderClient
}

func (p *GRPCProvider) DescribeSchema() *provider.DescribeSchemaOutput {
	return nil
}

func (p *GRPCProvider) ReadResource(*provider.ReadResourceInput) *provider.ReadResourceOutput {
	return nil
}

func (p *GRPCProvider) Stop() error {
	return nil
}

func (p *GRPCProvider) Shutdown() error {
	return nil
}
