package grpcwrap

import (
	"context"

	"github.com/4rchr4y/goray/interface/provider"
	"github.com/4rchr4y/goray/internal/proto/pluginproto"
)

type successor struct {
	pluginproto.UnimplementedProviderServer
	origin provider.Interface
}

func (p *successor) DescribeSchema(ctx context.Context, req *pluginproto.DescribeSchema_Request) (*pluginproto.DescribeSchema_Response, error) {
	return &pluginproto.DescribeSchema_Response{
		Provider: &pluginproto.Schema{
			Root: &pluginproto.Schema_Block{
				Attributes: []*pluginproto.Schema_Attribute{
					{
						Name: "go_version",
					},
				},
				Description: "Hello, World!",
			},
		},
	}, nil
}

func Successor(p provider.Interface) pluginproto.ProviderServer {
	return &successor{
		origin: p,
	}
}
