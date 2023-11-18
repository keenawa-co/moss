package compass

import (
	"errors"
	"go/ast"

	"github.com/4rchr4y/go-compass/obj"
)

func NewFuncDeclPicker() Picker {
	return NewPicker(pickFuncDecl)
}

func pickFuncDecl(state *State, node ast.Node) (obj.Object, error) {
	decl, _ := node.(*ast.FuncDecl)

	funcDeclObj, err := obj.NewFuncDeclObj(state.File, decl)
	if err != nil {
		return nil, errors.New("some error from pickFuncDecl 1") // TODO: add normal error return message
	}

	return &obj.DeclObj{
		Pos: state.File.FileSet.Position(decl.Pos()).Line,
		End: state.File.FileSet.Position(decl.End()).Line,
		Name: &obj.IdentObj{
			Name: decl.Name.Name,
			Kind: obj.Fun,
		},
		Type: funcDeclObj,
	}, nil
}
