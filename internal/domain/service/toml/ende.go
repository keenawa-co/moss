package toml

import (
	"io"

	"github.com/BurntSushi/toml"
)

type Encoder struct{}

func (Encoder) Encode(w io.Writer, v interface{}) error {
	return toml.NewEncoder(w).Encode(v)
}

type Decoder struct{}

func (Decoder) Decode(data string, v interface{}) (toml.MetaData, error) {
	return toml.Decode(data, v)
}
