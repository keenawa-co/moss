package convert

import (
	"encoding/json"

	"github.com/4rchr4y/godevkit/v3/must"
	"github.com/4rchr4y/goray/interface/driver"
	"github.com/4rchr4y/goray/internal/proto/protoschema"
	"github.com/4rchr4y/goray/internal/schematica"
)

var nestingModeMap = map[schematica.NestingMode]protoschema.Schema_NestingMode{}

func MustProviderSchema(s *driver.Schema) *protoschema.Schema {
	return &protoschema.Schema{
		Version: s.Version,
		Root:    MustSchemaBlock(s.Root),
	}
}

func MustSchemaBlock(block *schematica.Block) *protoschema.Schema_Block {
	result := &protoschema.Schema_Block{
		BlockTypes:  make([]*protoschema.Schema_NestedBlock, 0, len(block.BlockTypes)),
		Attributes:  make([]*protoschema.Schema_Attribute, 0, len(block.Attributes)),
		Description: block.Description,
		Deprecated:  block.Deprecated,
	}

	for name, a := range block.Attributes {
		result.Attributes = append(result.Attributes, processSchemaAttribute(name, a))
	}

	for name, b := range block.BlockTypes {
		result.BlockTypes = append(result.BlockTypes, &protoschema.Schema_NestedBlock{
			Name:    name,
			Block:   MustSchemaBlock(b.Block),
			Nesting: nestingModeMap[b.Nesting],
		})
	}

	return result
}

func SchemaObject(obj *schematica.Object) *protoschema.Schema_Object {
	result := &protoschema.Schema_Object{
		Attributes: make([]*protoschema.Schema_Attribute, 0, len(obj.Attributes)),
		Nesting:    nestingModeMap[obj.Nesting],
	}

	for name, a := range obj.Attributes {
		result.Attributes = append(result.Attributes, processSchemaAttribute(name, a))
	}

	return result
}

func processSchemaAttribute(name string, a *schematica.Attribute) *protoschema.Schema_Attribute {
	attr := &protoschema.Schema_Attribute{
		Name: name,
		Type: must.Must(json.Marshal(a.Type)),
		NestedType: func() *protoschema.Schema_Object {
			if a.NestingType == nil {
				return nil
			}

			return SchemaObject(a.NestingType)
		}(),
		Description: a.Description,
		Deprecated:  a.Deprecated,
		Optional:    a.Optional,
		Required:    a.Required,
	}

	return attr
}
