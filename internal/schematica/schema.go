package schematica

import (
	"github.com/4rchr4y/godevkit/v3/must"
	"github.com/hashicorp/hcl/v2/hcldec"
	"github.com/zclconf/go-cty/cty"
)

type NestingMode uint32

const (
	INVALID NestingMode = iota
)

var NestingModeToString = [...]string{
	INVALID: "invalid",
}

func (m NestingMode) String() string { return NestingModeToString[m] }

type Block struct {
	Attributes  map[string]*Attribute
	BlockTypes  map[string]*NestedBlock
	Description string
	Deprecated  bool
}

func (b *Block) Type() cty.Type {
	if b == nil {
		return cty.EmptyObject
	}

	return hcldec.ImpliedType(must.Must(DecodeBlock(b))).WithoutOptionalAttributesDeep()
}

type Attribute struct {
	Type        cty.Type
	NestedType  *Object
	Description string
	Required    bool
	Optional    bool
	Deprecated  bool
}

type Object struct {
	Attributes map[string]*Attribute
	Nesting    NestingMode
}

type NestedBlock struct {
	*Block
	Nesting NestingMode
}
