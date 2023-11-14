package picker

import (
	"errors"
	"go/ast"

	"github.com/4rchr4y/go-compass/obj"
	"github.com/4rchr4y/go-compass/state"
)

func NewFuncDeclPicker() Picker[obj.Object] {
	return NewPicker[obj.Object](pickFuncDecl)
}

func pickFuncDecl(state *state.State, node ast.Node) (obj.Object, error) {
	decl, _ := node.(*ast.FuncDecl)

	funcDeclObj, err := obj.NewFuncDeclObj(state.File, decl)
	if err != nil {
		return nil, errors.New("some error from pickFuncDecl 1") // TODO: add normal error return message
	}

	return &obj.DeclObj{
		Pos:  state.File.FileSet.Position(decl.Pos()).Line,
		End:  state.File.FileSet.Position(decl.End()).Line,
		Name: obj.NewIdentObj(decl.Name),
		Type: funcDeclObj,
	}, nil
}
