package toml

import "github.com/BurntSushi/toml"

type Decoder struct{}

func (Decoder) Decode(data string, v interface{}) (toml.MetaData, error) {
	return toml.Decode(data, v)
}
