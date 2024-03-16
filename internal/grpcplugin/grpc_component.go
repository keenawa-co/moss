package grpcplugin

import (
	"context"
	"errors"

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
	output := new(component.HeartbeatOutput)

	resp, err := p.client.Heartbeat(p.ctx, new(protocomponent.Heartbeat_Request))
	if err != nil {
		output.Diagnostics = output.Diagnostics.Append(convert.FromGRPCError(err))
		return output
	}

	return &component.HeartbeatOutput{
		Status:      convert.FromComponentProtoStatus[resp.Status],
		Diagnostics: output.Diagnostics.Append(convert.FromProtoDiagSet(resp.Diagnostics)),
	}
}

func (p *GRPCComponent) Configure(input *component.ConfigureInput) *component.ConfigureOutput {
	output := new(component.ConfigureOutput)

	resp, err := p.client.Configure(p.ctx, &protocomponent.Configure_Request{
		Msgpack: input.MessagePack,
	})
	if err != nil {
		output.Diagnostics = output.Diagnostics.Append(convert.FromGRPCError(err))
		return output
	}

	output.Diagnostics = output.Diagnostics.Append(convert.FromProtoDiagSet(resp.Diagnostics))
	return output
}

func (p *GRPCComponent) DescribeSchema() *component.DescribeSchemaOutput {
	output := new(component.DescribeSchemaOutput)

	resp, err := p.client.DescribeSchema(p.ctx, new(protocomponent.DescribeSchema_Request))
	if err != nil {
		output.Diagnostics = output.Diagnostics.Append(convert.FromGRPCError(err))
		return output
	}

	if resp.Driver == nil {
		output.Diagnostics = output.Diagnostics.Append(errors.New("missing provider schema"))
		return output
	}

	return &component.DescribeSchemaOutput{
		Schema:      convert.MustFromProtoComponentSchema(resp.Driver),
		Diagnostics: output.Diagnostics.Append(convert.FromGRPCError(err)),
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
