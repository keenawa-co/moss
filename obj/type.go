package obj

import (
	"go/ast"
	"go/token"
)

type EmbeddedObject interface{}

type TypeObj struct {
	Start        token.Pos
	End          token.Pos
	Name         *IdentObj
	Typ          any         // TODO: remove method Type() and rename this field to Type
	TypeParams   []*FieldObj // generic type params
	Dependencies map[string]int
}

func (o *TypeObj) EmbedObject(obj EmbeddedObject) {
	o.Typ = obj
}

func (o *TypeObj) Type() string {
	return "type"
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
		typeParams, err := processFieldList(fobj, ts.TypeParams.List, typeObj.ImportAdder)
		if err != nil {
			return nil, err
		}

		typeObj.TypeParams = typeParams
	}

	typeObj.Start = ts.Pos()
	typeObj.End = ts.End()
	typeObj.Name = NewIdentObj(ts.Name)

	return typeObj, nil
}
