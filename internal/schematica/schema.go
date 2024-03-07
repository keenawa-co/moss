package schematica

import "github.com/zclconf/go-cty/cty"

type NestingMode int

const (
	INVALID NestingMode = iota
)

var NestingModeToString = [...]string{
	INVALID: "invalid",
	1:       "invalid",
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
	NestingType *Object
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
