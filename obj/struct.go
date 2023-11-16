package obj

import (
	"fmt"
	"go/ast"
)

type StructTypeObj struct {
	Fields       *FieldObjList
	Dependencies map[string]int
	Incomplete   bool
}

func (o *StructTypeObj) ImportAdder(importIndex int, element string) {
	if o.Dependencies == nil {
		o.Dependencies = make(map[string]int)
	}

	o.Dependencies[element] = importIndex
}

func NewStructTypeObj(fobj *FileObj, structType *ast.StructType) (*StructTypeObj, error) {
	structTypeObj := new(StructTypeObj)

	if structType.Fields != nil && len(structType.Fields.List) > 0 {
		var err error
		structTypeObj.Fields, err = ProcessFieldList(fobj, structType.Fields, structTypeObj.ImportAdder)
		if err != nil {
			return nil, fmt.Errorf("failed to extract struct field map: %w", err)
		}
	}

	structTypeObj.Incomplete = structType.Incomplete
	return structTypeObj, nil
}
