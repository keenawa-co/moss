package schematica

import "github.com/zclconf/go-cty/cty"

type EmbeddedMode int

type BlockSpec struct {
	Attributes  map[string]*AttributeSpec
	BlockTypes  map[string]*NestedBlockSpec
	Description string
	Deprecated  bool
}

type AttributeSpec struct {
	Type         cty.Type
	EmbeddedType *ObjectSpec
	Description  string
	Required     bool
	Optional     bool
	Deprecated   bool
}

type ObjectSpec struct {
	Attributes map[string]*AttributeSpec
	Embedding  EmbeddedMode
}

type NestedBlockSpec struct {
	BlockSpec
	Embedding EmbeddedMode
}
