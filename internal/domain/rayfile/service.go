package rayfile

import (
	"io"
)

type rayfileTOMLDecoder interface {
	Decode(data string, v interface{}) error
}

type rayfileIoWrapper interface {
	ReadAll(reader io.Reader) ([]byte, error)
}

type RayfileService struct {
	toml rayfileTOMLDecoder
}

func (rs *RayfileService) Parse(iowrap rayfileIoWrapper, reader io.Reader) (*Rayfile, error) {
	content, err := iowrap.ReadAll(reader)
	if err != nil {
		return nil, err
	}

	rayfile := New()

	if err := rs.toml.Decode(string(content), rayfile); err != nil {
		return nil, err
	}

	return rayfile, nil
}

func NewRayfileService(ts rayfileTOMLDecoder) *RayfileService {
	return &RayfileService{
		toml: ts,
	}
}
