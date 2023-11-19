package obj

import (
	"fmt"
	"go/ast"
)

type TypeObj struct {
	Pos           int
	End           int
	Name          *IdentObj
	Type          any
	TypeParams    *FieldObjList
	DependsParams *FieldObjList
}

func (o *TypeObj) Kind() ObjKind {
	return o.Name.Kind
}

func (o *TypeObj) IsExported() bool {
	return o.Name.IsExported()
}

func (o *TypeObj) IsValid() bool {
	return o.Pos != noPos && o.End != noPos
}

func (o *TypeObj) ImportAdder(filed *FieldObj) {
	if o.DependsParams == nil {
		o.DependsParams = &FieldObjList{
			List: make([]*FieldObj, 0),
		}
	}

	o.DependsParams.List = append(o.DependsParams.List, filed)
}

// ----------------------------------------------------------------------------
// Struct type

// StructTypeObj encapsulates the details of a Go struct type in the AST.
// It provides a structure to hold the fields of the struct as well as any parameters
// on which the struct depends, indicating a relationship with other types.
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

// ----------------------------------------------------------------------------
// Function type

type FuncTypeObj struct {
	Params        *FieldObjList // incoming parameters
	TypeParams    *FieldObjList // generic parameters
	ResultParams  *FieldObjList // outgoing results
	DependsParams *FieldObjList // object dependencies
}

func (o *FuncTypeObj) ImportAdder(filed *FieldObj) {
	if o.DependsParams == nil {
		o.DependsParams = &FieldObjList{
			List: make([]*FieldObj, 0),
		}
	}

	o.DependsParams.List = append(o.DependsParams.List, filed)
}

func NewFuncTypeObj(fobj *FileObj, funcType *ast.FuncType) (*FuncTypeObj, error) {
	funcTypeObj := new(FuncTypeObj)

	if funcType.Params != nil && len(funcType.Params.List) > 0 {
		var err error
		funcTypeObj.Params, err = ProcessFieldList(fobj, funcType.Params, funcTypeObj.ImportAdder)
		if err != nil {
			return nil, fmt.Errorf("failed to extract func params list: %w", err)
		}
	}

	if funcType.TypeParams != nil && len(funcType.TypeParams.List) > 0 {
		var err error
		funcTypeObj.TypeParams, err = ProcessFieldList(fobj, funcType.TypeParams, funcTypeObj.ImportAdder)
		if err != nil {
			return nil, fmt.Errorf("failed to extract func type params list: %w", err)
		}
	}

	if funcType.Results != nil && len(funcType.Results.List) > 0 {
		var err error
		funcTypeObj.ResultParams, err = ProcessFieldList(fobj, funcType.Results, funcTypeObj.ImportAdder)
		if err != nil {
			return nil, fmt.Errorf("failed to extract func results params list: %w", err)
		}
	}

	return funcTypeObj, nil
}
