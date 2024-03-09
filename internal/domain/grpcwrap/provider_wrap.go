package grpcwrap

import (
	"context"

	"github.com/4rchr4y/goray/interface/provider"
	"github.com/4rchr4y/goray/internal/proto/convert"
	"github.com/4rchr4y/goray/internal/proto/pluginproto"
)

// schema -> proto

type successor struct {
	pluginproto.UnimplementedProviderServer
	origin provider.Interface
	schema *provider.DescribeSchemaOutput
}

func Successor(p provider.Interface) pluginproto.ProviderServer {
	return &successor{
		origin: p,
		schema: p.DescribeSchema(),
	}
}

func (p *successor) DescribeSchema(_ context.Context, req *pluginproto.DescribeSchema_Request) (*pluginproto.DescribeSchema_Response, error) {
	resp := &pluginproto.DescribeSchema_Response{
		Provider: &pluginproto.Schema{
			Root: &pluginproto.Schema_Block{},
		},
	}

	if p.schema.Schema.Root != nil {
		resp.Provider = convert.MustProviderSchema(p.schema.Schema)
	}

	return resp, nil
}

func (s *successor) Stop(ctx context.Context, req *pluginproto.Stop_Request) (*pluginproto.Stop_Response, error) {
	resp := &pluginproto.Stop_Response{}
	if err := s.origin.Stop(); err != nil {
		resp.Error = err.Error()
	}

	return resp, nil
}
