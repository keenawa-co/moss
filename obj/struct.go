package obj

import (
	"errors"
	"fmt"
	"go/ast"
)

type (
	StructObjMeta struct {
		LineCount int
	}

	StructTypeObj struct {
		Name         *string
		Fields       []*FieldObj
		Dependencies map[string]int
		Incomplete   bool
		Valid        bool
		Metadata     *StructObjMeta
	}
)

func (o *StructTypeObj) Type() string {
	return "struct"
}

func (o *StructTypeObj) Adder(importIndex int, element string) {
	if o.Dependencies == nil {
		o.Dependencies = make(map[string]int)
	}

	o.Dependencies[element] = importIndex
}

// TODO: get rid of name arg
func NewStructObj(fobj *FileObj, node ast.Node, name *string) (*StructTypeObj, error) {
	structTypeObj := new(StructTypeObj)

	var structType *ast.StructType
	switch n := node.(type) {
	case *ast.TypeSpec:
		var ok bool
		structType, ok = n.Type.(*ast.StructType)
		if !ok {
			return nil, errors.New("TypeSpec node does not contain a StructType")
		}

	case *ast.StructType:
		structType = n

	default:
		return nil, errors.New("node is not a TypeSpec or StructType")
	}

	fieldList, err := processFieldList(fobj, structType.Fields.List, structTypeObj.Adder)
	if err != nil {
		return nil, fmt.Errorf("failed to extract struct field map: %w", err)
	}

	structTypeObj.Name = name
	structTypeObj.Fields = fieldList
	structTypeObj.Incomplete = structType.Incomplete
	structTypeObj.Valid = structType.Struct.IsValid()

	return structTypeObj, nil
}

type UsedPackage struct {
	_              [0]int
	Alias, Element string
}
