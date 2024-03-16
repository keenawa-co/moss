package convert

import (
	"encoding/json"

	"github.com/4rchr4y/godevkit/v3/must"
	"github.com/4rchr4y/goray/interface/component"
	"github.com/4rchr4y/goray/interface/driver"
	"github.com/4rchr4y/goray/internal/proto/protopkg"
	"github.com/4rchr4y/goray/internal/schematica"
)

var FromSchemaNestingMode = [...]protopkg.Schema_NestingMode{
	schematica.INVALID: protopkg.Schema_INVALID,
}

var FromProtoSchemaNestingMode = [...]schematica.NestingMode{
	protopkg.Schema_INVALID: schematica.INVALID,
}

// Panic here indicates a mismatch between the types in the
// protocol and in the code. Should never happen.
var (
	_ = [1]int{}[len(FromSchemaNestingMode)-len(FromProtoSchemaNestingMode)]
	_ = [1]int{}[len(FromProtoSchemaNestingMode)-len(FromSchemaNestingMode)]
)

func MustFromProtoDriverSchema(s *protopkg.Schema) *driver.Schema {
	return &driver.Schema{
		Version: s.Version,
		Root:    MustFromProtoSchemaBlock(s.Root),
	}
}

func MustFromProtoComponentSchema(s *protopkg.Schema) *component.Schema {
	return &component.Schema{
		Version: s.Version,
		Root:    MustFromProtoSchemaBlock(s.Root),
	}
}

func MustFromProtoSchemaBlock(block *protopkg.Schema_Block) *schematica.Block {
	result := &schematica.Block{
		BlockTypes:  make(map[string]*schematica.NestedBlock, len(block.BlockTypes)),
		Attributes:  make(map[string]*schematica.Attribute, len(block.Attributes)),
		Description: block.Description,
		Deprecated:  block.Deprecated,
	}

	for _, a := range block.Attributes {
		result.Attributes[a.Name] = processFromProtoSchemaAttribute(a)
	}

	for _, b := range block.BlockTypes {
		result.BlockTypes[b.Name] = &schematica.NestedBlock{
			Block:   MustFromProtoSchemaBlock(b.Block),
			Nesting: FromProtoSchemaNestingMode[b.Nesting],
		}
	}

	return result
}

func FromProtoSchemaObject(obj *protopkg.Schema_Object) *schematica.Object {
	if obj == nil {
		return nil
	}

	result := &schematica.Object{
		Attributes: make(map[string]*schematica.Attribute, len(obj.Attributes)),
		Nesting:    FromProtoSchemaNestingMode[obj.Nesting],
	}

	for _, a := range obj.Attributes {
		result.Attributes[a.Name] = processFromProtoSchemaAttribute(a)
	}

	return result
}

func processFromProtoSchemaAttribute(a *protopkg.Schema_Attribute) *schematica.Attribute {
	attr := &schematica.Attribute{
		Description: a.Description,
		Required:    a.Required,
		Optional:    a.Optional,
		NestedType:  FromProtoSchemaObject(a.NestedType),
		Deprecated:  a.Deprecated,
	}

	if err := json.Unmarshal(a.Type, &attr.Type); err != nil {
		panic(err)
	}

	return attr
}

func MustFromDriverSchema(s *driver.Schema) *protopkg.Schema {
	return &protopkg.Schema{
		Version: s.Version,
		Root:    MustFromSchemaBlock(s.Root),
	}
}

func MustFromComponentSchema(s *component.Schema) *protopkg.Schema {
	return &protopkg.Schema{
		Version: s.Version,
		Root:    MustFromSchemaBlock(s.Root),
	}
}

func MustFromSchemaBlock(block *schematica.Block) *protopkg.Schema_Block {
	result := &protopkg.Schema_Block{
		BlockTypes:  make([]*protopkg.Schema_NestedBlock, 0, len(block.BlockTypes)),
		Attributes:  make([]*protopkg.Schema_Attribute, 0, len(block.Attributes)),
		Description: block.Description,
		Deprecated:  block.Deprecated,
	}

	for name, a := range block.Attributes {
		result.Attributes = append(result.Attributes, processFromSchemaAttribute(name, a))
	}

	for name, b := range block.BlockTypes {
		result.BlockTypes = append(result.BlockTypes, &protopkg.Schema_NestedBlock{
			Name:    name,
			Block:   MustFromSchemaBlock(b.Block),
			Nesting: FromSchemaNestingMode[b.Nesting],
		})
	}

	return result
}

func FromSchemaObject(obj *schematica.Object) *protopkg.Schema_Object {
	result := &protopkg.Schema_Object{
		Attributes: make([]*protopkg.Schema_Attribute, 0, len(obj.Attributes)),
		Nesting:    FromSchemaNestingMode[obj.Nesting],
	}

	for name, a := range obj.Attributes {
		result.Attributes = append(result.Attributes, processFromSchemaAttribute(name, a))
	}

	return result
}

func processFromSchemaAttribute(name string, a *schematica.Attribute) *protopkg.Schema_Attribute {
	attr := &protopkg.Schema_Attribute{
		Name: name,
		Type: must.Must(json.Marshal(a.Type)),
		NestedType: func() *protopkg.Schema_Object {
			if a.NestedType == nil {
				return nil
			}

			return FromSchemaObject(a.NestedType)
		}(),
		Description: a.Description,
		Deprecated:  a.Deprecated,
		Optional:    a.Optional,
		Required:    a.Required,
	}

	return attr
}
