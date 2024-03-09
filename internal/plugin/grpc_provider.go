package plugin

import (
	"context"
	"errors"
	"fmt"

	"github.com/4rchr4y/goray/interface/provider"
	"github.com/4rchr4y/goray/internal/proto/convert"
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

// proto -> schema

type GRPCProvider struct {
	PluginClient *pluginHCL.Client
	ctx          context.Context
	client       pluginproto.ProviderClient
}

func (p *GRPCProvider) DescribeSchema() *provider.DescribeSchemaOutput {
	output := &provider.DescribeSchemaOutput{}

	descSchemaResp, err := p.client.DescribeSchema(p.ctx, new(pluginproto.DescribeSchema_Request))
	if err != nil {
		//TODO: response.Diagnostics.Append() <- error
		fmt.Println(err)
		return output
	}

	if descSchemaResp.Provider == nil {
		fmt.Println("missing provider schema")
		// output.Diagnostics = output.Diagnostics.Append(errors.New("missing provider schema"))
		return output
	}

	output.Schema = convert.ProtoSchema(descSchemaResp.Provider)

	return output
}

func (p *GRPCProvider) ReadResource(*provider.ReadResourceInput) *provider.ReadResourceOutput {
	return nil
}

func (p *GRPCProvider) Stop() error {
	resp, err := p.client.Stop(p.ctx, new(pluginproto.Stop_Request))
	if err != nil {
		return err
	}

	if resp.Error != "" {
		return errors.New(resp.Error)
	}

	return nil
}

func (p *GRPCProvider) Shutdown() error {
	p.PluginClient.Kill()
	return nil
}
