package grpcwrap

import (
	"context"

	"github.com/4rchr4y/goray/interface/component"
	"github.com/4rchr4y/goray/internal/proto/convert"
	"github.com/4rchr4y/goray/internal/proto/protocomponent"
)

type componentWrapper struct {
	protocomponent.UnimplementedComponentServer
	origin component.Interface
}

func ComponentWrapper(c component.Interface) protocomponent.ComponentServer {
	return &componentWrapper{
		origin: c,
	}
}

func (p *componentWrapper) Heartbeat(_ context.Context, req *protocomponent.Heartbeat_Request) (*protocomponent.Heartbeat_Response, error) {
	output := p.origin.Heartbeat()

	return &protocomponent.Heartbeat_Response{
		Status:      convert.ToComponentProtoStatus[output.Status],
		Diagnostics: convert.ToProtoDiagSet(output.Diagnostics),
	}, nil
}

func (p *componentWrapper) Configure(_ context.Context, req *protocomponent.Configure_Request) (*protocomponent.Configure_Response, error) {
	output := p.origin.Configure(&component.ConfigureInput{
		MessagePack: req.Msgpack,
	})

	return &protocomponent.Configure_Response{
		Diagnostics: convert.ToProtoDiagSet(output.Diagnostics),
	}, nil
}

func (p *componentWrapper) DescribeSchema(_ context.Context, req *protocomponent.DescribeSchema_Request) (*protocomponent.DescribeSchema_Response, error) {
	output := p.origin.DescribeSchema()

	return &protocomponent.DescribeSchema_Response{
		Driver:      convert.MustFromComponentSchema(output.Schema),
		Diagnostics: convert.ToProtoDiagSet(output.Diagnostics),
	}, nil
}

func (s *componentWrapper) Stop(ctx context.Context, req *protocomponent.Stop_Request) (*protocomponent.Stop_Response, error) {
	resp := &protocomponent.Stop_Response{}
	if err := s.origin.Stop(); err != nil {
		resp.Error = err.Error()
	}

	return resp, nil
}
