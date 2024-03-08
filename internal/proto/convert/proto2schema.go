package convert

import (
	"encoding/json"

	"github.com/4rchr4y/goray/interface/provider"
	"github.com/4rchr4y/goray/internal/proto/pluginproto"
	"github.com/4rchr4y/goray/internal/schematica"
)

var protoNestingModeMap = map[pluginproto.Schema_NestingMode]schematica.NestingMode{}

func ProtoSchema(s *pluginproto.Schema) *provider.Schema {
	return &provider.Schema{
		Version: s.Version,
		Root:    ProtoSchemaBlock(s.Root),
	}
}

func ProtoSchemaBlock(block *pluginproto.Schema_Block) *schematica.Block {
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
			Block:   ProtoSchemaBlock(b.Block),
			Nesting: protoNestingModeMap[b.Nesting],
		}
	}

	return result
}

func ProtoSchemaObject(obj *pluginproto.Schema_Object) *schematica.Object {
	result := &schematica.Object{
		Attributes: make(map[string]*schematica.Attribute, len(obj.Attributes)),
		Nesting:    protoNestingModeMap[obj.Nesting],
	}

	for _, a := range obj.Attributes {
		result.Attributes[a.Name] = processProtoSchemaAttribute(a)
	}

	return result
}

func processProtoSchemaAttribute(a *pluginproto.Schema_Attribute) *schematica.Attribute {
	attr := &schematica.Attribute{
		Description: a.Description,
		Required:    a.Required,
		Optional:    a.Optional,
		NestingType: ProtoSchemaObject(a.NestedType),
		Deprecated:  a.Deprecated,
	}

	if err := json.Unmarshal(a.Type, &attr.Type); err != nil {
		panic(err)
	}

	return attr
}
