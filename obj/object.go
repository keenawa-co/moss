package obj

import (
	"go/ast"
	"go/token"
)

const noPos int = 0

// Custom type based on *ast.Object
// Need for specification of collected object types
type ObjKind int

const (
	Bad ObjKind = iota // for error handling
	Pkg                // package
	Fle                // file
	Imp                // import
	Con                // constant
	Typ                // type
	Var                // variable
	Fun                // function or method
	Lbl                // label
)

var objKindStrings = [...]string{
	Bad: "bad",
	Pkg: "package",
	Fle: "file",
	Imp: "import",
	Con: "const",
	Typ: "type",
	Var: "var",
	Fun: "func",
	Lbl: "label",
}

func (kind ObjKind) String() string { return objKindStrings[kind] }

type Object interface {
	Kind() ObjKind
	IsValid() bool
	IsExported() bool
}

// ----------------------------------------------------------------------------
// Ident type

// Object represents an identifier
type IdentObj struct {
	Name string
	Kind ObjKind
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

// ----------------------------------------------------------------------------
// Filed

type FieldObj struct {
	Names []*IdentObj
	Type  any
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

// Len returns the number of parameters or struct fields represented by a FieldList.
func (f *FieldObjList) Len() int {
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

// ----------------------------------------------------------------------------
// Block statement type

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
