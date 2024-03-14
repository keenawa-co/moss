package schematica

import (
	"errors"
	"fmt"

	"github.com/hashicorp/hcl/v2/hcldec"
	"github.com/zclconf/go-cty/cty"
)

func DecodeBlock(block *Block) (hcldec.Spec, error) {
	result := hcldec.ObjectSpec{}

	if block == nil {
		return result, nil
	}

	for name, attr := range block.Attributes {
		a, err := DecodeAttribute(name, attr)
		if err != nil {
			return nil, fmt.Errorf("invalid attribute: %v", err)
		}

		result[name] = a
	}

	for name, b := range block.BlockTypes {
		if _, exists := result[name]; exists {
			return nil, errors.New("block and attribute with same name")
		}

		child, err := DecodeBlock(b.Block)
		if err != nil {
			return nil, fmt.Errorf("invalid nested block: %v", err)
		}

		// TODO:
		// switch b.Nesting {
		// default: // should never happen
		// 	continue
		// }

		result[name] = &hcldec.BlockSpec{
			TypeName: name,
			Nested:   child,
		}
	}

	return result, nil
}

func DecodeAttribute(name string, attr *Attribute) (hcldec.Spec, error) {
	if attr == nil || (attr.Type == cty.NilType && attr.NestedType == nil) {
		return nil, errors.New("schema is nil")
	}

	result := &hcldec.AttrSpec{
		Name: name,
	}

	if attr.NestedType != nil {
		if attr.Type != cty.NilType {
			return nil, errors.New("NestedType and Type cannot both be set")
		}

		result.Type = DecodeObject(attr.NestedType)
		result.Required = attr.Required
		return result, nil
	}

	result.Type = attr.Type
	result.Required = attr.Required
	return result, nil
}

func DecodeObject(obj *Object) (result cty.Type) {
	if obj == nil {
		return cty.EmptyObject
	}

	attributes := make(map[string]cty.Type, len(obj.Attributes))
	optional := make([]string, 0)
	for name, attr := range obj.Attributes {
		if attr.Optional {
			optional = append(optional, name)
		}

		if attr.NestedType != nil {
			attributes[name] = DecodeObject(attr.NestedType)
		} else {
			attributes[name] = attr.Type
		}
	}

	if len(optional) > 0 {
		result = cty.ObjectWithOptionalAttrs(attributes, optional)
	} else {
		result = cty.Object(attributes)
	}

	// TODO:
	// switch obj.Nesting {
	// }

	return result

}
