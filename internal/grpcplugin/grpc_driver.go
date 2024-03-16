package grpcplugin

import (
	"context"
	"errors"
	"fmt"

	"github.com/4rchr4y/goray/interface/driver"
	"github.com/4rchr4y/goray/internal/proto/convert"
	"github.com/4rchr4y/goray/internal/proto/protodriver"
	"google.golang.org/grpc"

	pluginHCL "github.com/hashicorp/go-plugin"
)

type GRPCDriverPlugin struct {
	pluginHCL.Plugin
	ServeFn DriverServeFn
}

func (p *GRPCDriverPlugin) GRPCClient(ctx context.Context, broker *pluginHCL.GRPCBroker, c *grpc.ClientConn) (interface{}, error) {
	return &GRPCDriver{
		client: protodriver.NewDriverClient(c),
		ctx:    ctx,
	}, nil
}

func (p *GRPCDriverPlugin) GRPCServer(broker *pluginHCL.GRPCBroker, s *grpc.Server) error {
	protodriver.RegisterDriverServer(s, p.ServeFn())
	return nil
}

// proto -> schema

type GRPCDriver struct {
	PluginClient *pluginHCL.Client
	ctx          context.Context
	client       protodriver.DriverClient
}

func (p *GRPCDriver) DescribeSchema() *driver.DescribeSchemaOutput {
	output := &driver.DescribeSchemaOutput{}

	descSchemaResp, err := p.client.DescribeSchema(p.ctx, new(protodriver.DescribeSchema_Request))
	if err != nil {
		//TODO: response.Diagnostics.Append() <- error
		fmt.Println(err)
		return output
	}

	if descSchemaResp.Driver == nil {
		fmt.Println("missing provider schema")
		// output.Diagnostics = output.Diagnostics.Append(errors.New("missing provider schema"))
		return output
	}

	output.Schema = convert.MustFromProtoDriverSchema(descSchemaResp.Driver)

	return output
}

func (p *GRPCDriver) ReadResource(*driver.ReadResourceInput) *driver.ReadResourceOutput {
	return nil
}

func (p *GRPCDriver) Stop() error {
	resp, err := p.client.Stop(p.ctx, new(protodriver.Stop_Request))
	if err != nil {
		return err
	}

	if resp.Error != "" {
		return errors.New(resp.Error)
	}

	return nil
}

func (p *GRPCDriver) Shutdown() error {
	p.PluginClient.Kill()
	return nil
}

var _ driver.Interface = (*GRPCDriver)(nil)
