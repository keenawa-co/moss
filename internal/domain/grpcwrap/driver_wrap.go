package grpcwrap

import (
	"context"

	"github.com/4rchr4y/goray/interface/driver"
	"github.com/4rchr4y/goray/internal/proto/convert"
	"github.com/4rchr4y/goray/internal/proto/protodriver"
	"github.com/4rchr4y/goray/internal/proto/protoschema"
)

// schema -> proto

type successor struct {
	protodriver.UnimplementedDriverServer
	origin driver.Interface
	schema *driver.DescribeSchemaOutput
}

func Successor(p driver.Interface) protodriver.DriverServer {
	return &successor{
		origin: p,
		schema: p.DescribeSchema(),
	}
}

func (p *successor) DescribeSchema(_ context.Context, req *protodriver.DescribeSchema_Request) (*protodriver.DescribeSchema_Response, error) {
	resp := &protodriver.DescribeSchema_Response{
		Driver: &protoschema.Schema{
			Root: &protoschema.Schema_Block{},
		},
	}

	if p.schema.Schema.Root != nil {
		resp.Driver = convert.MustProviderSchema(p.schema.Schema)
	}

	return resp, nil
}

func (s *successor) Stop(ctx context.Context, req *protodriver.Stop_Request) (*protodriver.Stop_Response, error) {
	resp := &protodriver.Stop_Response{}
	if err := s.origin.Stop(); err != nil {
		resp.Error = err.Error()
	}

	return resp, nil
}
