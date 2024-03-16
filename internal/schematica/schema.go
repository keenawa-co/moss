package schematica

import (
	"github.com/4rchr4y/goray/internal/proto/protopkg"
	"github.com/zclconf/go-cty/cty"
)

type NestingMode int

const (
	INVALID NestingMode = iota
)

var NestingModeToString = [...]string{
	INVALID: "invalid",
}

func (m NestingMode) String() string { return NestingModeToString[m] }

// The panic here indicates a mismatch between the types in the
// protocol and in the code. Should never happen.
var (
	_ = [1]int{}[len(protopkg.Schema_NestingMode_name)-len(NestingModeToString)]
	_ = [1]int{}[len(NestingModeToString)-len(protopkg.Schema_NestingMode_name)]
)

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
