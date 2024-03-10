package grpcplugin

import (
	"context"
	"errors"
	"fmt"

	"github.com/4rchr4y/goray/interface/component"
	"github.com/4rchr4y/goray/internal/proto/convert"
	"github.com/4rchr4y/goray/internal/proto/protocomponent"
	pluginHCL "github.com/hashicorp/go-plugin"
	"google.golang.org/grpc"
)

type GRPCComponentPlugin struct {
	pluginHCL.Plugin
	ServeFn ComponentServeFn
}

func (p *GRPCComponentPlugin) GRPCClient(ctx context.Context, broker *pluginHCL.GRPCBroker, c *grpc.ClientConn) (interface{}, error) {
	return &GRPCComponent{
		client: protocomponent.NewComponentClient(c),
		ctx:    ctx,
	}, nil
}

func (p *GRPCComponentPlugin) GRPCServer(broker *pluginHCL.GRPCBroker, s *grpc.Server) error {
	protocomponent.RegisterComponentServer(s, p.ServeFn())
	return nil
}

type GRPCComponent struct {
	PluginClient *pluginHCL.Client
	ctx          context.Context
	client       protocomponent.ComponentClient
}

func (p *GRPCComponent) Heartbeat() *component.HeartbeatOutput {
	heartbeatResp, err := p.client.Heartbeat(p.ctx, new(protocomponent.Heartbeat_Request))
	if err != nil {
		//TODO:
		fmt.Println(err)
		return nil
	}

	return &component.HeartbeatOutput{
		Status: heartbeatResp.Status,
	}
}

func (p *GRPCComponent) DescribeSchema() *component.DescribeSchemaOutput {
	descSchemaResp, err := p.client.DescribeSchema(p.ctx, new(protocomponent.DescribeSchema_Request))
	if err != nil {
		//TODO: response.Diagnostics.Append() <- error
		fmt.Println(err)
		return nil
	}

	if descSchemaResp.Driver == nil {
		fmt.Println("missing provider schema")
		// output.Diagnostics = output.Diagnostics.Append(errors.New("missing provider schema"))
		return nil
	}

	return &component.DescribeSchemaOutput{
		Schema: convert.MustProtoComponentSchema(descSchemaResp.Driver),
	}
}

func (p *GRPCComponent) Stop() error {
	resp, err := p.client.Stop(p.ctx, new(protocomponent.Stop_Request))
	if err != nil {
		return err
	}

	if resp.Error != "" {
		return errors.New(resp.Error)
	}

	return nil
}

func (p *GRPCComponent) Shutdown() error {
	p.PluginClient.Kill()
	return nil
}

var _ component.Interface = (*GRPCComponent)(nil)
