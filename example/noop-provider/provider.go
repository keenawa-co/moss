package noop_provider

import (
	"github.com/4rchr4y/goray/interface/provider"
	"github.com/4rchr4y/goray/internal/schematica"
	"github.com/zclconf/go-cty/cty"
)

type NoopProviderServer struct {
	// pluginproto.UnimplementedProviderServer
}

func Provider() provider.Interface {
	return &NoopProviderServer{}
}

func (s *NoopProviderServer) DescribeSchema() *provider.DescribeSchemaOutput {
	return &provider.DescribeSchemaOutput{
		Schema: &provider.Schema{
			Version: 1,
			Root: &schematica.Block{
				Attributes: map[string]*schematica.Attribute{
					"value": {
						Optional: true,
						Type:     cty.String,
					},
				},
				Description: "Hello, Ray!",
			},
		},
	}
}

func (s *NoopProviderServer) ReadResource(*provider.ReadResourceInput) *provider.ReadResourceOutput {
	return &provider.ReadResourceOutput{}
}

func (s *NoopProviderServer) Stop() error {
	return nil
}

func (s *NoopProviderServer) Shutdown() error {
	return nil
}

// type NoopProviderPlugin struct {
// 	pluginHCL.Plugin
// 	Impl *NoopProviderServer
// }

// func (p *NoopProviderPlugin) GRPCServer(broker *pluginHCL.GRPCBroker, s *grpc.Server) error {
// 	pluginproto.RegisterProviderServer(s, p.Impl)
// 	return nil
// }

// func (p *NoopProviderPlugin) GRPCClient(ctx context.Context, broker *pluginHCL.GRPCBroker, c *grpc.ClientConn) (interface{}, error) {
// 	return pluginproto.NewProviderClient(c), nil
// }
