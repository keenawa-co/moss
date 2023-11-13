package obj

import (
	"fmt"
	"go/ast"
	"go/token"
)

type StructTypeObj struct {
	Struct       token.Pos // position of "struct" keyword
	Fields       *FieldObjList
	Dependencies map[string]int
	Incomplete   bool
}

func (o *StructTypeObj) IsValid() bool {
	return o.Struct != token.NoPos
}

func (o *StructTypeObj) ImportAdder(importIndex int, element string) {
	if o.Dependencies == nil {
		o.Dependencies = make(map[string]int)
	}

	o.Dependencies[element] = importIndex
}

func NewStructObj(fobj *FileObj, structType *ast.StructType) (*StructTypeObj, error) {
	structTypeObj := new(StructTypeObj)
	fieldList, err := processFieldList(fobj, structType.Fields, structTypeObj.ImportAdder)
	if err != nil {
		return nil, fmt.Errorf("failed to extract struct field map: %w", err)
	}

	structTypeObj.Fields = fieldList
	structTypeObj.Incomplete = structType.Incomplete
	structTypeObj.Struct = structType.Struct

	return structTypeObj, nil
}
