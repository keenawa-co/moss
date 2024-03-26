package hcllang

import (
	"fmt"
	"os"

	"github.com/zclconf/go-cty/cty"
	"github.com/zclconf/go-cty/cty/function"
)

var GetEnvFn = function.New(&function.Spec{
	Params: []function.Parameter{
		{
			Name: "key",
			Type: cty.String,
		},
	},
	Type: function.StaticReturnType(cty.String),
	Impl: func(args []cty.Value, retType cty.Type) (cty.Value, error) {
		key := args[0].AsString()
		value, ok := os.LookupEnv(key)
		fmt.Println("+++++++++++++++++++", value)
		if !ok {
			return cty.UnknownVal(cty.String), fmt.Errorf("environment variable %s was not found", key)
		}

		return cty.StringVal(value), nil
	},
})
