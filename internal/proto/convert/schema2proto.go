package convert

import (
	"encoding/json"

	"github.com/4rchr4y/godevkit/v3/must"
	"github.com/4rchr4y/goray/interface/component"
	"github.com/4rchr4y/goray/interface/driver"
	"github.com/4rchr4y/goray/internal/proto/protopkg"

	"github.com/4rchr4y/goray/internal/schematica"
)

var nestingModeMap = map[schematica.NestingMode]protopkg.Schema_NestingMode{}

func MustDriverSchema(s *driver.Schema) *protopkg.Schema {
	return &protopkg.Schema{
		Version: s.Version,
		Root:    MustSchemaBlock(s.Root),
	}
}

func MustComponentSchema(s *component.Schema) *protopkg.Schema {
	return &protopkg.Schema{
		Version: s.Version,
		Root:    MustSchemaBlock(s.Root),
	}
}

func MustSchemaBlock(block *schematica.Block) *protopkg.Schema_Block {
	result := &protopkg.Schema_Block{
		BlockTypes:  make([]*protopkg.Schema_NestedBlock, 0, len(block.BlockTypes)),
		Attributes:  make([]*protopkg.Schema_Attribute, 0, len(block.Attributes)),
		Description: block.Description,
		Deprecated:  block.Deprecated,
	}

	for name, a := range block.Attributes {
		result.Attributes = append(result.Attributes, processSchemaAttribute(name, a))
	}

	for name, b := range block.BlockTypes {
		result.BlockTypes = append(result.BlockTypes, &protopkg.Schema_NestedBlock{
			Name:    name,
			Block:   MustSchemaBlock(b.Block),
			Nesting: nestingModeMap[b.Nesting],
		})
	}

	return result
}

func SchemaObject(obj *schematica.Object) *protopkg.Schema_Object {
	result := &protopkg.Schema_Object{
		Attributes: make([]*protopkg.Schema_Attribute, 0, len(obj.Attributes)),
		Nesting:    nestingModeMap[obj.Nesting],
	}

	for name, a := range obj.Attributes {
		result.Attributes = append(result.Attributes, processSchemaAttribute(name, a))
	}

	return result
}

func processSchemaAttribute(name string, a *schematica.Attribute) *protopkg.Schema_Attribute {
	attr := &protopkg.Schema_Attribute{
		Name: name,
		Type: must.Must(json.Marshal(a.Type)),
		NestedType: func() *protopkg.Schema_Object {
			if a.NestedType == nil {
				return nil
			}

			return SchemaObject(a.NestedType)
		}(),
		Description: a.Description,
		Deprecated:  a.Deprecated,
		Optional:    a.Optional,
		Required:    a.Required,
	}

	return attr
}
