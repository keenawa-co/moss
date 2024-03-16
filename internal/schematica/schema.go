package schematica

import (
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
