package obj

import (
	"fmt"
	"go/ast"
)

type StructTypeObj struct {
	Fields        *FieldObjList
	DependsParams *FieldObjList
	Incomplete    bool
}

func (o *StructTypeObj) ImportAdder(filed *FieldObj) {
	if o.DependsParams == nil {
		o.DependsParams = &FieldObjList{
			List: make([]*FieldObj, 0),
		}
	}

	o.DependsParams.List = append(o.DependsParams.List, filed)
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
