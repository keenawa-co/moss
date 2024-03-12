package convert

import (
	"encoding/json"

	"github.com/4rchr4y/goray/interface/component"
	"github.com/4rchr4y/goray/interface/driver"
	"github.com/4rchr4y/goray/internal/proto/protoschema"
	"github.com/4rchr4y/goray/internal/schematica"
)

var protoNestingModeMap = map[protoschema.Schema_NestingMode]schematica.NestingMode{}

func MustProtoDriverSchema(s *protoschema.Schema) *driver.Schema {
	return &driver.Schema{
		Version: s.Version,
		Root:    MustProtoSchemaBlock(s.Root),
	}
}

func MustProtoComponentSchema(s *protoschema.Schema) *component.Schema {
	return &component.Schema{
		Version: s.Version,
		Root:    MustProtoSchemaBlock(s.Root),
	}
}

func MustProtoSchemaBlock(block *protoschema.Schema_Block) *schematica.Block {
	result := &schematica.Block{
		BlockTypes:  make(map[string]*schematica.NestedBlock, len(block.BlockTypes)),
		Attributes:  make(map[string]*schematica.Attribute, len(block.Attributes)),
		Description: block.Description,
		Deprecated:  block.Deprecated,
	}

	for _, a := range block.Attributes {
		result.Attributes[a.Name] = processProtoSchemaAttribute(a)
	}

	for _, b := range block.BlockTypes {
		result.BlockTypes[b.Name] = &schematica.NestedBlock{
			Block:   MustProtoSchemaBlock(b.Block),
			Nesting: protoNestingModeMap[b.Nesting],
		}
	}

	return result
}

func ProtoSchemaObject(obj *protoschema.Schema_Object) *schematica.Object {
	if obj == nil {
		return nil
	}

	result := &schematica.Object{
		Attributes: make(map[string]*schematica.Attribute, len(obj.Attributes)),
		Nesting:    protoNestingModeMap[obj.Nesting],
	}

	for _, a := range obj.Attributes {
		result.Attributes[a.Name] = processProtoSchemaAttribute(a)
	}

	return result
}

func processProtoSchemaAttribute(a *protoschema.Schema_Attribute) *schematica.Attribute {
	attr := &schematica.Attribute{
		Description: a.Description,
		Required:    a.Required,
		Optional:    a.Optional,
		NestedType:  ProtoSchemaObject(a.NestedType),
		Deprecated:  a.Deprecated,
	}

	if err := json.Unmarshal(a.Type, &attr.Type); err != nil {
		panic(err)
	}

	return attr
}
