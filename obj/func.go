package obj

import (
	"fmt"
	"go/ast"
	"go/token"
)

type FuncTypeObj struct {
	Params       []*FieldObj
	Results      map[string]*FieldObj // TODO: Not implemented
	Dependencies map[string]int
}

func (o *FuncTypeObj) ImportAdder(importIndex int, element string) {
	if o.Dependencies == nil {
		o.Dependencies = make(map[string]int)
	}

	o.Dependencies[element] = importIndex
}

func NewFuncTypeObj(fobj *FileObj, funcType *ast.FuncType) (*FuncTypeObj, error) {
	funcTypeObj := new(FuncTypeObj)

	paramList, err := processFieldList(fobj, funcType.Params.List, funcTypeObj.ImportAdder)
	if err != nil {
		return nil, fmt.Errorf("failed to extract func param list: %w", err)
	}

	funcTypeObj.Params = paramList
	return funcTypeObj, nil
}

type FuncDeclBodyObj struct {
	FieldAccess map[string]int
	Stmt        *BlockStmtObj
}

func (o *FuncDeclBodyObj) FieldAdder(fieldName string) {
	if o.FieldAccess == nil {
		o.FieldAccess = make(map[string]int)
	}

	o.FieldAccess[fieldName]++

}

type FuncDeclObj struct {
	Recv      []*FieldObj
	Name      *IdentObj
	Typ       *FuncTypeObj // TODO: remove method Type() and rename this field to Type
	Body      *FuncDeclBodyObj
	Recursive bool
}

func (o *FuncDeclObj) IsExported() bool {
	return token.IsExported(o.Name.Name)
}

func inspectBody(fobj *FileObj, obj *FuncDeclObj, body *ast.BlockStmt) *FuncDeclBodyObj {
	bodyObj := &FuncDeclBodyObj{
		Stmt: &BlockStmtObj{
			Rbrace: body.Rbrace,
			Lbrace: body.Lbrace,
		},
	}

	ast.Inspect(body, func(n ast.Node) bool {
		switch expr := n.(type) {
		case *ast.SelectorExpr:
			handleSelectorExpr(fobj, bodyObj, obj, expr)
		case *ast.CallExpr:
			handleCallExpr(obj, expr)
		}
		return true
	})

	return bodyObj
}

func handleSelectorExpr(fobj *FileObj, body *FuncDeclBodyObj, obj *FuncDeclObj, expr *ast.SelectorExpr) {
	ident, ok := expr.X.(*ast.Ident)
	if !ok {
		return
	}

	if obj.Recv[0].Names[0].Name == ident.Name {
		body.FieldAdder(expr.Sel.Name)
		return
	}

	if index, exists := fobj.Entities.Imports.InternalImportsMeta[ident.Name]; exists {
		body.Stmt.ImportAdder(index, expr.Sel.Name)
		return
	}
}

func handleCallExpr(obj *FuncDeclObj, expr *ast.CallExpr) {
	// checking for direct recursion in regular functions
	if ident, ok := expr.Fun.(*ast.Ident); ok {
		if ident.Name == obj.Name.Name {
			obj.Recursive = true
		}
		return
	}

	// checking for recursion in structure methods
	if sel, ok := expr.Fun.(*ast.SelectorExpr); ok {
		ident, ok := sel.X.(*ast.Ident)
		if !ok {
			return
		}

		// check that the called method is of the same type as the recipient
		if obj.Recv[0].Names[0].Name != ident.Name {
			return
		}

		if sel.Sel.Name != obj.Name.Name {
			return
		}

		obj.Recursive = true
	}
}

func receiverDefinition(fobj *FileObj, decl *ast.FuncDecl) ([]*FieldObj, error) {
	if decl.Recv == nil || len(decl.Recv.List) == 0 {
		return nil, nil
	}

	fieldObjList, err := processFieldList(fobj, decl.Recv.List, nil)
	if err != nil {
		return nil, err
	}

	return fieldObjList, nil
}

func (o *FuncDeclObj) Type() string {
	return "func"
}

func NewFuncDeclObj(fobj *FileObj, decl *ast.FuncDecl) (*FuncDeclObj, error) {
	funcDeclObj := new(FuncDeclObj)
	funcDeclObj.Name = NewIdentObj(decl.Name)

	receiver, err := receiverDefinition(fobj, decl)
	if err != nil {
		return nil, err
	}

	funcTypeObj, err := NewFuncTypeObj(fobj, decl.Type)
	if err != nil {
		return nil, err
	}

	funcDeclObj.Recv = receiver
	funcDeclObj.Typ = funcTypeObj
	funcDeclObj.Body = inspectBody(fobj, funcDeclObj, decl.Body)

	return funcDeclObj, nil
}
