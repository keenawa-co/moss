package obj

import (
	"go/ast"
	"go/token"
)

// ---------------------------- Position --------------------------- //

type (
	// Pos is the line number position in the source code file.
	Pos int

	// Position is an array storing the start and end position of
	// an object in the source code file.
	Position   [2]Pos
	Positioner interface {
		// Start position of an object in a source code file.
		Pos() Pos
		// End position of an object in a source code file.
		End() Pos
	}
)

const (
	// The zero value for Pos is NoPos; there is no file and line information
	// associated with it. NoPos is always smaller than any other Pos value.
	NoPos Pos = 0
)

func (s Position) Pos() Pos { return s[0] }
func (s Position) End() Pos { return s[0] }

func NewPosition(fset *token.FileSet, node ast.Node) *Position {
	return &Position{
		Pos(fset.Position(node.Pos()).Line),
		Pos(fset.Position(node.End()).Line),
	}
}

// ----------------------------- Object --------------------------- //

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

// ------------------------------ Ident ---------------------------- //

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

// ------------------------------ Field ---------------------------- //

type FieldObj struct {
	Names []*IdentObj
	Type  any
}

type FieldObjList struct {
	List []*FieldObj
}

// TODO: add tests
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

// TODO: add tests
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

// ------------------------- Block Statement ----------------------- //

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
