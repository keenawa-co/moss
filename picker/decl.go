package picker

import (
	"go/ast"

	"github.com/4rchr4y/go-compass/obj"
	"github.com/4rchr4y/go-compass/state"
)

func NewFuncDeclPicker() Picker[obj.Object] {
	return NewPicker[obj.Object](pickFuncDecl)
}

func pickFuncDecl(state *state.State, node ast.Node) (obj.Object, error) {
	funcDecl, _ := node.(*ast.FuncDecl)

	funcDeclObj, err := obj.NewFuncDeclObj(state.File, funcDecl)
	if err != nil {
		return nil, err
	}

	// TODO: use IdentObj as name arg
	declObj := obj.NewDeclObj(state.File.FileSet, node, funcDeclObj, funcDeclObj.Name.Name)
	return declObj, nil
}
