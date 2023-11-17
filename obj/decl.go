package obj

import "go/ast"

type DeclObj struct {
	Pos  int
	End  int
	Name *IdentObj
	Type any
}

func (o *DeclObj) Kind() ObjKind {
	return o.Name.Kind
}

func (o *DeclObj) IsExported() bool {
	return o.Name.IsExported()
}

func (o *DeclObj) IsValid() bool {
	return o.Pos != NoPos && o.End != NoPos
}

// ----------------------------------------------------------------------------
// Function declaration

// FuncDeclObj is a custom representation of an AST function declaration.
// It encapsulates the receiver, name, type, and body of a function
// as well as a flag indicating whether the function is recursive.
//
// The structure mirrors the components of a Go function declaration,
// providing a more accessible way to interact with and analyze function details.
type FuncDeclObj struct {
	Recv      *FieldObjList
	Name      *IdentObj
	Type      *FuncTypeObj
	Body      *FuncDeclBodyObj
	Recursive bool
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

func NewFuncDeclObj(fobj *FileObj, decl *ast.FuncDecl) (*FuncDeclObj, error) {
	funcDeclObj := &FuncDeclObj{
		Name: &IdentObj{
			Name: decl.Name.Name,
			Kind: Fun,
		},
	}

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

	if _, exists := fobj.Imports.Meta[ident.Name]; exists {
		body.Stmt.ImportAdder(&FieldObj{
			Names: []*IdentObj{{Name: ident.Name}},
			Type:  expr.Sel.Name,
		})
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
