package convert

import (
	"github.com/zclconf/go-cty/cty"
	"github.com/zclconf/go-cty/cty/msgpack"
)

func DecodeValue(val []byte, ty cty.Type) (v cty.Value, err error) {
	v = cty.NullVal(ty)
	if val == nil {
		return v, nil
	}

	return msgpack.Unmarshal(val, ty)
}

func EncodeValue(val cty.Value, ty cty.Type) ([]byte, error) {
	return msgpack.Marshal(val, ty)
}
