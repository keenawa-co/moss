package rayfile

import (
	"io"
)

type tomlService interface {
	Decode(data string, v interface{}) error
}

type ioWrapper interface {
	ReadAll(reader io.Reader) ([]byte, error)
}

type RayfileService struct {
	toml tomlService
}

func (rs *RayfileService) Parse(iowrap ioWrapper, reader io.Reader) (*Rayfile, error) {
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

func NewRayfileService(ts tomlService) *RayfileService {
	return &RayfileService{
		toml: ts,
	}
}
