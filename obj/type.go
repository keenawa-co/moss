package obj

import (
	"errors"
	"go/ast"
	"go/token"

	"github.com/4rchr4y/go-compass/utils"
)

type TypeObj struct {
	Pos          token.Pos
	End          token.Pos
	Name         *IdentObj
	Type         any
	TypeParams   *FieldObjList
	Dependencies map[string]int
	Loc          int
}

func (o *TypeObj) IsValid() bool {
	return o.Pos != token.NoPos && o.End != token.NoPos
}

func (o *TypeObj) IsExported() bool {
	return o.Name.IsExported()
}

func (o *TypeObj) ImportAdder(importIndex int, element string) {
	if o.Dependencies == nil {
		o.Dependencies = make(map[string]int)
	}

	o.Dependencies[element] = importIndex
}

func NewTypeObj(fobj *FileObj, ts *ast.TypeSpec) (*TypeObj, error) {
	typeObj := new(TypeObj)

	if ts.TypeParams != nil {
		typeParams, err := processFieldList(fobj, ts.TypeParams, typeObj.ImportAdder)
		if err != nil {
			return nil, err
		}

		typeObj.TypeParams = typeParams
	}

	typ, err := typeDefinition(fobj, ts)
	if err != nil {
		return nil, errors.New("some error from NewTypeObj 3") // TODO: add normal error return message
	}

	typeObj.Pos = ts.Pos()
	typeObj.End = ts.End()
	typeObj.Name = NewIdentObj(ts.Name)
	typeObj.Type = typ
	typeObj.Loc = utils.CalcNodeLOC(fobj.FileSet, ts)

	return typeObj, nil
}

func typeDefinition(fobj *FileObj, ts *ast.TypeSpec) (any, error) {
	switch typ := ts.Type.(type) {
	case *ast.StructType:
		return NewStructObj(fobj, typ)

	case *ast.FuncType:
		return NewFuncTypeObj(fobj, typ)
	}

	return nil, errors.New("unknown type")
}
