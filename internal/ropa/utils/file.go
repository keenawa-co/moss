package utils

import (
	"fmt"
	"io"

	"github.com/4rchr4y/goray/internal/ropa/types"
)

type tomlEncoder interface {
	Encode(w io.Writer, v interface{}) error
}

func EncodeBPMFile(ts tomlEncoder, w io.Writer, file types.BPMFile) error {
	if err := ts.Encode(w, file); err != nil {
		return fmt.Errorf("failed to encode %s file: %v", file.Name(), err)
	}

	return nil
}

type ioWrapper interface {
	ReadAll(r io.Reader) ([]byte, error)
}

type tomlDecoder interface {
	Decode(data string, v interface{}) error
}

func DecodeBPMFile[F types.BPMFile](iowrap ioWrapper, toml tomlDecoder, reader io.Reader) (*F, error) {
	content, err := iowrap.ReadAll(reader)
	if err != nil {
		return nil, err
	}

	file := new(F)
	if err := toml.Decode(string(content), file); err != nil {
		return nil, err
	}

	return file, nil
}
