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
	funcDecl, _ := node.(*ast.FuncDecl)

	funcDeclObj, err := obj.NewFuncDeclObj2(state.File, funcDecl)
	if err != nil {
		return nil, err
	}

	// TODO: use IdentObj as name arg
	declObj := obj.NewDeclObj(state.File.FileSet, node, funcDeclObj, funcDeclObj.Name.Name)
	return declObj, nil
}

func getParentStruct(funcDecl *ast.FuncDecl) (*ast.Ident, error) {
	if funcDecl.Recv == nil || len(funcDecl.Recv.List) == 0 {
		return nil, nil
	}

	receiverType := funcDecl.Recv.List[0].Type

	switch t := receiverType.(type) {
	case *ast.StarExpr:
		// if the receiver's type is a pointer, attempt to get the identifier of the struct
		if ident, ok := t.X.(*ast.Ident); ok {
			return ident, nil
		}

	case *ast.Ident:
		// receiver's type is not a pointer, it's a regular struct,
		// so return the identifier of the struct
		return t, nil
	}

	return nil, errors.New("invalid receiver type in method declaration")
}
