package convert

import (
	"encoding/json"

	"github.com/4rchr4y/godevkit/v3/must"
	"github.com/4rchr4y/goray/interface/provider"
	"github.com/4rchr4y/goray/internal/proto/pluginproto"
	"github.com/4rchr4y/goray/internal/schematica"
)

var nestingModeMap = map[schematica.NestingMode]pluginproto.Schema_NestingMode{}

func MustProviderSchema(s *provider.Schema) *pluginproto.Schema {
	return &pluginproto.Schema{
		Version: s.Version,
		Root:    MustSchemaBlock(s.Root),
	}
}

func MustSchemaBlock(block *schematica.Block) *pluginproto.Schema_Block {
	result := &pluginproto.Schema_Block{
		BlockTypes:  make([]*pluginproto.Schema_NestedBlock, 0, len(block.BlockTypes)),
		Attributes:  make([]*pluginproto.Schema_Attribute, 0, len(block.Attributes)),
		Description: block.Description,
		Deprecated:  block.Deprecated,
	}

	for name, a := range block.Attributes {
		result.Attributes = append(result.Attributes, processSchemaAttribute(name, a))
	}

	for name, b := range block.BlockTypes {
		result.BlockTypes = append(result.BlockTypes, &pluginproto.Schema_NestedBlock{
			Name:    name,
			Block:   MustSchemaBlock(b.Block),
			Nesting: nestingModeMap[b.Nesting],
		})
	}

	return result
}

func SchemaObject(obj *schematica.Object) *pluginproto.Schema_Object {
	result := &pluginproto.Schema_Object{
		Attributes: make([]*pluginproto.Schema_Attribute, 0, len(obj.Attributes)),
		Nesting:    nestingModeMap[obj.Nesting],
	}

	for name, a := range obj.Attributes {
		result.Attributes = append(result.Attributes, processSchemaAttribute(name, a))
	}

	return result
}

func processSchemaAttribute(name string, a *schematica.Attribute) *pluginproto.Schema_Attribute {
	attr := &pluginproto.Schema_Attribute{
		Name:        name,
		Type:        must.Must(json.Marshal(a.Type)),
		NestedType:  SchemaObject(a.NestingType),
		Description: a.Description,
		Deprecated:  a.Deprecated,
		Optional:    a.Optional,
		Required:    a.Required,
	}

	return attr
}
