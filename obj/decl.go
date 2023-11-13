package obj

import (
	"go/ast"
	"go/token"

	"github.com/4rchr4y/go-compass/utils"
)

type DeclObj struct {
	Name string // TODO: should be a Ident
	Type Object
	Loc  int
}

// func (o *DeclObj) Pos() token.Pos {
// 	return o.Type.Pos()
// }

// func (o *DeclObj) End() token.Pos {
// 	return o.Type.End()
// }

// func (o *DeclObj) IsValid() bool {
// 	return o.Pos() != token.NoPos && o.End() != token.NoPos
// }

// TODO: make a real func
func (o *DeclObj) IsExported() bool {
	return true
}

func NewDeclObj(fset *token.FileSet, node ast.Node, obj Object, name string) *DeclObj {
	return &DeclObj{
		Name: name,
		Type: obj,
		Loc:  utils.CalcNodeLOC(fset, node),
	}
}
