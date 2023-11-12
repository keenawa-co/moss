package obj

import (
	"go/ast"
	"go/token"
)

type DeclObj struct {
	Start token.Pos
	End   token.Pos
	Name  string
	Obj   Object
	// TODO: Introduce the use of internal object markings.
	// Stop using reflection in groups of anatomyizers and link everything to internal types of AST
	Typ AstTyp
	Loc int
}

func (o *DeclObj) Type() string {
	return "decl"
}

func NewDeclObj(fset *token.FileSet, node ast.Node, obj Object, name string) *DeclObj {
	return &DeclObj{
		Start: node.Pos(),
		End:   node.End(),
		Name:  name,
		Typ:   0,
		Obj:   obj,
		Loc:   CalcNodeLOC(fset, node),
	}
}
