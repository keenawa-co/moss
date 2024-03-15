package grpcwrap

import (
	"context"

	"github.com/4rchr4y/goray/interface/component"
	"github.com/4rchr4y/goray/internal/proto/convert"
	"github.com/4rchr4y/goray/internal/proto/protocomponent"
	"github.com/4rchr4y/goray/internal/proto/protoschema"
)

type componentWrapper struct {
	protocomponent.UnimplementedComponentServer
	origin component.Interface
	schema *component.DescribeSchemaOutput
}

func ComponentWrapper(c component.Interface) protocomponent.ComponentServer {
	return &componentWrapper{
		origin: c,
		schema: c.DescribeSchema(),
	}
}

func (p *componentWrapper) Heartbeat(_ context.Context, req *protocomponent.Heartbeat_Request) (*protocomponent.Heartbeat_Response, error) {
	output := p.origin.Heartbeat()

	resp := new(protocomponent.Heartbeat_Response)
	if output.Error != nil {
		resp.Error = output.Error.Error()
		return resp, nil
	}

	return &protocomponent.Heartbeat_Response{
		Status: p.origin.Heartbeat().Status,
	}, nil
}

func (p *componentWrapper) Configure(_ context.Context, req *protocomponent.Configure_Request) (*protocomponent.Configure_Response, error) {
	output := p.origin.Configure(&component.ConfigureInput{
		MessagePack: req.Msgpack,
	})

	resp := new(protocomponent.Configure_Response)
	if output.Error != nil {
		resp.Error = output.Error.Error()
		return resp, nil
	}

	return resp, nil
}

func (p *componentWrapper) DescribeSchema(_ context.Context, req *protocomponent.DescribeSchema_Request) (*protocomponent.DescribeSchema_Response, error) {
	resp := &protocomponent.DescribeSchema_Response{
		Driver: &protoschema.Schema{
			Root: &protoschema.Schema_Block{},
		},
	}

	if p.schema.Schema.Root != nil {
		resp.Driver = convert.MustComponentSchema(p.schema.Schema)
	}

	return resp, nil
}

func (s *componentWrapper) Stop(ctx context.Context, req *protocomponent.Stop_Request) (*protocomponent.Stop_Response, error) {
	resp := &protocomponent.Stop_Response{}
	if err := s.origin.Stop(); err != nil {
		resp.Error = err.Error()
	}

	return resp, nil
}
