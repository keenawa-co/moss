package convert

import (
	"encoding/json"

	"github.com/4rchr4y/godevkit/v3/must"
	"github.com/4rchr4y/goray/interface/provider"
	"github.com/4rchr4y/goray/internal/proto/pluginproto"
	"github.com/4rchr4y/goray/internal/schematica"
)

func MustProviderSchema(s *provider.Schema) *pluginproto.Schema {
	return &pluginproto.Schema{
		Version: s.Version,
		Root:    MustSchemaBlock(s.Root),
	}
}

func MustSchemaBlock(block *schematica.Block) *pluginproto.Schema_Block {
	result := &pluginproto.Schema_Block{
		BlockTypes:  make([]*pluginproto.Schema_EmbeddedBlock, 0, len(block.BlockTypes)),
		Attributes:  make([]*pluginproto.Schema_Attribute, 0, len(block.Attributes)),
		Description: block.Description,
		Deprecated:  block.Deprecated,
	}

	for name, a := range block.Attributes {
		result.Attributes = append(result.Attributes, processSchemaAttribute(name, a))
	}

	for name, b := range block.BlockTypes {
		eb := &pluginproto.Schema_EmbeddedBlock{
			Name:  name,
			Block: MustSchemaBlock(b.Block),
		}

		switch b.Embedding {
		default:
			eb.Embedding = pluginproto.Schema_EmbeddedBlock_INVALID
		}

		result.BlockTypes = append(result.BlockTypes, eb)
	}

	return result
}

func SchemaObject(obj *schematica.Object) *pluginproto.Schema_Object {
	result := &pluginproto.Schema_Object{
		Attributes: make([]*pluginproto.Schema_Attribute, 0, len(obj.Attributes)),
	}

	switch obj.Embedding {
	default:
		result.Embedding = pluginproto.Schema_Object_INVALID
	}

	for name, a := range obj.Attributes {
		result.Attributes = append(result.Attributes, processSchemaAttribute(name, a))
	}

	return result
}

func processSchemaAttribute(name string, a *schematica.Attribute) *pluginproto.Schema_Attribute {
	attr := &pluginproto.Schema_Attribute{
		Name:         name,
		Type:         must.Must(json.Marshal(a.Type)),
		EmbeddedType: SchemaObject(a.EmbeddedType),
		Description:  a.Description,
		Deprecated:   a.Deprecated,
		Optional:     a.Optional,
		Required:     a.Required,
	}

	return attr
}
