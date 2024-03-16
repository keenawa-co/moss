package grpcwrap

import (
	"context"

	"github.com/4rchr4y/goray/interface/driver"
	"github.com/4rchr4y/goray/internal/proto/convert"
	"github.com/4rchr4y/goray/internal/proto/protodriver"
	"github.com/4rchr4y/goray/internal/proto/protopkg"
)

// schema -> proto

type driverWrapper struct {
	protodriver.UnimplementedDriverServer
	origin driver.Interface
	schema *driver.DescribeSchemaOutput
}

func DriverWrapper(p driver.Interface) protodriver.DriverServer {
	return &driverWrapper{
		origin: p,
		schema: p.DescribeSchema(),
	}
}

func (p *driverWrapper) DescribeSchema(_ context.Context, req *protodriver.DescribeSchema_Request) (*protodriver.DescribeSchema_Response, error) {
	resp := &protodriver.DescribeSchema_Response{
		Driver: &protopkg.Schema{
			Root: &protopkg.Schema_Block{},
		},
	}

	if p.schema.Schema.Root != nil {
		resp.Driver = convert.MustDriverSchema(p.schema.Schema)
	}

	return resp, nil
}

func (s *driverWrapper) Stop(ctx context.Context, req *protodriver.Stop_Request) (*protodriver.Stop_Response, error) {
	resp := &protodriver.Stop_Response{}
	if err := s.origin.Stop(); err != nil {
		resp.Error = err.Error()
	}

	return resp, nil
}

var _ protodriver.DriverServer = (*driverWrapper)(nil)
