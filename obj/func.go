package obj

import (
	"fmt"
	"go/ast"
)

type FuncTypeObj struct {
	Params       *FieldObjList // (incoming) parameters
	TypeParams   *FieldObjList
	Results      map[string]*FieldObj // (outgoing) results TODO: Not implemented
	Dependencies map[string]int       // TODO: make it *FieldObjList type
}

func (o *FuncTypeObj) ImportAdder(importIndex int, element string) {
	if o.Dependencies == nil {
		o.Dependencies = make(map[string]int)
	}

	o.Dependencies[element] = importIndex
}

func NewFuncTypeObj(fobj *FileObj, funcType *ast.FuncType) (*FuncTypeObj, error) {
	funcTypeObj := new(FuncTypeObj)

	if funcType.Params != nil && len(funcType.Params.List) > 0 {
		var err error
		funcTypeObj.Params, err = ProcessFieldList(fobj, funcType.Params, funcTypeObj.ImportAdder)
		if err != nil {
			return nil, fmt.Errorf("failed to extract func params list: %w", err)
		}
	}

	if funcType.TypeParams != nil && len(funcType.Params.List) > 0 {
		var err error
		funcTypeObj.TypeParams, err = ProcessFieldList(fobj, funcType.TypeParams, funcTypeObj.ImportAdder)
		if err != nil {
			return nil, fmt.Errorf("failed to extract func type params list: %w", err)
		}
	}

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
	Recv      *FieldObjList
	Name      *IdentObj
	Type      *FuncTypeObj
	Body      *FuncDeclBodyObj
	Recursive bool
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
	funcDeclObj.Type = funcTypeObj
	funcDeclObj.Body = inspectBody(fobj, funcDeclObj, decl.Body)

	return funcDeclObj, nil
}

func inspectBody(fobj *FileObj, obj *FuncDeclObj, body *ast.BlockStmt) *FuncDeclBodyObj {
	bodyObj := &FuncDeclBodyObj{
		Stmt: new(BlockStmtObj),
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

	if obj.Recv.List[0].Names[0].Name == ident.Name {
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
		if obj.Recv.List[0].Names[0].Name != ident.Name {
			return
		}

		if sel.Sel.Name != obj.Name.Name {
			return
		}

		obj.Recursive = true
	}
}

func receiverDefinition(fobj *FileObj, decl *ast.FuncDecl) (*FieldObjList, error) {
	if decl.Recv == nil || len(decl.Recv.List) < 1 {
		return nil, nil
	}

	fieldObjList, err := ProcessFieldList(fobj, decl.Recv, nil)
	if err != nil {
		return nil, err
	}

	return fieldObjList, nil
}
