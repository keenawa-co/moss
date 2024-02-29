package hcllang

import "github.com/zclconf/go-cty/cty"

type EmbeddedMode int

type Block struct {
	Attributes  map[string]*Attribute
	BlockTypes  map[string]*NestedBlock
	Description string
	Deprecated  bool
}

type Attribute struct {
	Type         cty.Type
	EmbeddedType *Object
	Description  string
	Required     bool
	Optional     bool
	Deprecated   bool
}

type Object struct {
	Attributes map[string]*Attribute
	Embedding  EmbeddedMode
}

type NestedBlock struct {
	Block
	Embedding EmbeddedMode
}
