package obj

import (
	"bytes"
	"fmt"
	"go/ast"
	"go/format"
	"go/token"
	"reflect"
)

type IdentObj struct {
	Name string
}

func (id *IdentObj) String() string {
	if id != nil {
		return id.Name
	}

	return "<nil>"
}

func (idObj *IdentObj) IsExported() bool {
	return token.IsExported(idObj.Name)
}

func NewIdentObj(id *ast.Ident) *IdentObj {
	return &IdentObj{
		Name: id.Name,
	}
}

type FieldObjList struct {
	List []*FieldObj
}

func (f *FieldObjList) FindByName(name string) []*FieldObj {
	result := make([]*FieldObj, 0)

	for _, filed := range f.List {
		for _, filedName := range filed.Names {
			if filedName.Name != name {
				continue
			}

			result = append(result, filed)
		}
	}

	return result
}

func (f *FieldObjList) FindByType(typ any) []*FieldObj {
	result := make([]*FieldObj, 0)

	for _, filed := range f.List {
		if filed.Type != typ {
			continue
		}

		result = append(result, filed)
	}

	return result
}

// Length returns the number of parameters or struct fields represented by a FieldList.
func (f *FieldObjList) Length() int {
	n := 0

	for _, g := range f.List {
		m := len(g.Names)
		if m == 0 {
			m = 1
		}
		n += m
	}

	return n
}

type FieldObj struct {
	Names []*IdentObj
	Type  any
}

type BlockStmtObj struct {
	DependsParams *FieldObjList
}

func (o *BlockStmtObj) ImportAdder(filed *FieldObj) {
	if o.DependsParams == nil {
		o.DependsParams = &FieldObjList{
			List: make([]*FieldObj, 0),
		}
	}

	o.DependsParams.List = append(o.DependsParams.List, filed)
}

func determineExprType(fobj *FileObj, expr ast.Expr, adder func(filed *FieldObj)) (any, error) {
	switch e := expr.(type) {
	case *ast.StructType:
		return NewStructTypeObj(fobj, e)

	case *ast.SelectorExpr:
		ident, ok := e.X.(*ast.Ident)
		if !ok {
			return e.Sel.Name, nil
		}

		if _, exists := fobj.Entities.Imports.Meta[ident.Name]; exists {
			adder(&FieldObj{
				Names: []*IdentObj{{Name: ident.Name}},
				Type:  e.Sel.Name,
			})
		}

		return e.Sel.Name, nil

	case *ast.StarExpr:
		// This branch, in my understanding, is used only to determine the receiver of methods.
		// Getting into this branch means that receiver's type is a pointer.

		// attempt to get the name of the struct
		if ident, ok := e.X.(*ast.Ident); ok {
			return ident.Name, nil
		}

		return nil, fmt.Errorf("failed to get expr %s type", reflect.TypeOf(e.X))

	case *ast.Ident:
		// This branch, in my understanding, is used only to determine the receiver of methods.
		// Getting into this branch means that receiver's type is not a pointer, it's a regular struct.
		// So return the name of the struct.
		return e.Name, nil

	default:
		var buf bytes.Buffer
		if err := format.Node(&buf, fobj.FileSet, e); err != nil {
			return nil, fmt.Errorf("failed to format expr: %w", err)
		}

		return buf.String(), nil
	}
}

func processField(fobj *FileObj, field *ast.Field, adder func(filed *FieldObj)) (*FieldObj, error) {
	typ, err := determineExprType(fobj, field.Type, adder)
	if err != nil {
		return nil, err
	}

	names := make([]*IdentObj, 0, len(field.Names))
	for _, name := range field.Names {
		names = append(names, NewIdentObj(name))
	}

	return &FieldObj{
		Names: names,
		Type:  typ,
	}, nil
}

func ProcessFieldList(fobj *FileObj, fieldList *ast.FieldList, adder func(filed *FieldObj)) (*FieldObjList, error) {
	fieldObjList := &FieldObjList{
		List: make([]*FieldObj, 0, len(fieldList.List)),
	}

	for _, field := range fieldList.List {
		fieldObj, err := processField(fobj, field, adder)
		if err != nil {
			return nil, err
		}

		fieldObjList.List = append(fieldObjList.List, fieldObj)
	}

	return fieldObjList, nil
}
