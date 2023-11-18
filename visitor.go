package compass

import (
	"fmt"
	"go/ast"
	"sync"
)

type Visitor interface {
	// Custom implementation of a standard ast.Walk function.
	Visit(ctx *State, node ast.Node) (w Visitor)
}

type visitor struct {
	noCopy noCopy

	// Created map of analyzers for a specific file
	pickerGroup PickerGroup

	once sync.Once
}

func NewVisitor(group PickerFactoryGroup) *visitor {
	v := new(visitor)
	v.once.Do(func() {
		v.pickerGroup = group.Make()
	})

	return v
}

func (v *visitor) Visit(state *State, node ast.Node) Visitor {
	if node == nil {
		return v
	}

	analyzer, ok := v.pickerGroup.Search(node)
	if !ok {
		return v
	}

	object, err := analyzer.Pick(state, node)
	if err != nil {
		fmt.Println(err) // TODO: decide later how to handle the error
		return v
	}

	if err := state.File.Save(object); err != nil {
		fmt.Println(err) // TODO: decide later how to handle the error
	}

	return v
}

func walk(state *State, v Visitor, node ast.Node) {
	if v = v.Visit(state, node); v == nil {
		return
	}

	// walk children
	// (the order of the cases matches the order
	// of the corresponding node types in ast.go)
	switch n := node.(type) {
	// Comments and fields
	case *ast.Comment:
		// nothing to do

	case *ast.CommentGroup:
		for _, c := range n.List {
			walk(state, v, c)
		}

	case *ast.Field:
		if n.Doc != nil {
			walk(state, v, n.Doc)
		}
		walkIdentList(state, v, n.Names)
		if n.Type != nil {
			walk(state, v, n.Type)
		}
		if n.Tag != nil {
			walk(state, v, n.Tag)
		}
		if n.Comment != nil {
			walk(state, v, n.Comment)
		}

	case *ast.FieldList:
		for _, f := range n.List {
			walk(state, v, f)
		}

	// Expressions
	case *ast.BadExpr, *ast.Ident, *ast.BasicLit:
		// nothing to do

	case *ast.Ellipsis:
		if n.Elt != nil {
			walk(state, v, n.Elt)
		}

	case *ast.FuncLit:
		walk(state, v, n.Type)
		walk(state, v, n.Body)

	case *ast.CompositeLit:
		if n.Type != nil {
			walk(state, v, n.Type)
		}
		walkExprList(state, v, n.Elts)

	case *ast.ParenExpr:
		walk(state, v, n.X)

	case *ast.SelectorExpr:
		walk(state, v, n.X)
		walk(state, v, n.Sel)

	case *ast.IndexExpr:
		walk(state, v, n.X)
		walk(state, v, n.Index)

	case *ast.IndexListExpr:
		walk(state, v, n.X)
		for _, index := range n.Indices {
			walk(state, v, index)
		}

	case *ast.SliceExpr:
		walk(state, v, n.X)
		if n.Low != nil {
			walk(state, v, n.Low)
		}
		if n.High != nil {
			walk(state, v, n.High)
		}
		if n.Max != nil {
			walk(state, v, n.Max)
		}

	case *ast.TypeAssertExpr:
		walk(state, v, n.X)
		if n.Type != nil {
			walk(state, v, n.Type)
		}

	case *ast.CallExpr:
		walk(state, v, n.Fun)
		walkExprList(state, v, n.Args)

	case *ast.StarExpr:
		walk(state, v, n.X)

	case *ast.UnaryExpr:
		walk(state, v, n.X)

	case *ast.BinaryExpr:
		walk(state, v, n.X)
		walk(state, v, n.Y)

	case *ast.KeyValueExpr:
		walk(state, v, n.Key)
		walk(state, v, n.Value)

	// Types
	case *ast.ArrayType:
		if n.Len != nil {
			walk(state, v, n.Len)
		}
		walk(state, v, n.Elt)

	case *ast.StructType:
		walk(state, v, n.Fields)

	case *ast.FuncType:
		if n.TypeParams != nil {
			walk(state, v, n.TypeParams)
		}
		if n.Params != nil {
			walk(state, v, n.Params)
		}
		if n.Results != nil {
			walk(state, v, n.Results)
		}

	case *ast.InterfaceType:
		walk(state, v, n.Methods)

	case *ast.MapType:
		walk(state, v, n.Key)
		walk(state, v, n.Value)

	case *ast.ChanType:
		walk(state, v, n.Value)

	// Statements
	case *ast.BadStmt:
		// nothing to do

	case *ast.DeclStmt:
		walk(state, v, n.Decl)

	case *ast.EmptyStmt:
		// nothing to do

	case *ast.LabeledStmt:
		walk(state, v, n.Label)
		walk(state, v, n.Stmt)

	case *ast.ExprStmt:
		walk(state, v, n.X)

	case *ast.SendStmt:
		walk(state, v, n.Chan)
		walk(state, v, n.Value)

	case *ast.IncDecStmt:
		walk(state, v, n.X)

	case *ast.AssignStmt:
		walkExprList(state, v, n.Lhs)
		walkExprList(state, v, n.Rhs)

	case *ast.GoStmt:
		walk(state, v, n.Call)

	case *ast.DeferStmt:
		walk(state, v, n.Call)

	case *ast.ReturnStmt:
		walkExprList(state, v, n.Results)

	case *ast.BranchStmt:
		if n.Label != nil {
			walk(state, v, n.Label)
		}

	case *ast.BlockStmt:
		walkStmtList(state, v, n.List)

	case *ast.IfStmt:
		if n.Init != nil {
			walk(state, v, n.Init)
		}
		walk(state, v, n.Cond)
		walk(state, v, n.Body)
		if n.Else != nil {
			walk(state, v, n.Else)
		}

	case *ast.CaseClause:
		walkExprList(state, v, n.List)
		walkStmtList(state, v, n.Body)

	case *ast.SwitchStmt:
		if n.Init != nil {
			walk(state, v, n.Init)
		}
		if n.Tag != nil {
			walk(state, v, n.Tag)
		}
		walk(state, v, n.Body)

	case *ast.TypeSwitchStmt:
		if n.Init != nil {
			walk(state, v, n.Init)
		}
		walk(state, v, n.Assign)
		walk(state, v, n.Body)

	case *ast.CommClause:
		if n.Comm != nil {
			walk(state, v, n.Comm)
		}
		walkStmtList(state, v, n.Body)

	case *ast.SelectStmt:
		walk(state, v, n.Body)

	case *ast.ForStmt:
		if n.Init != nil {
			walk(state, v, n.Init)
		}
		if n.Cond != nil {
			walk(state, v, n.Cond)
		}
		if n.Post != nil {
			walk(state, v, n.Post)
		}
		walk(state, v, n.Body)

	case *ast.RangeStmt:
		if n.Key != nil {
			walk(state, v, n.Key)
		}
		if n.Value != nil {
			walk(state, v, n.Value)
		}
		walk(state, v, n.X)
		walk(state, v, n.Body)

	// Declarations
	case *ast.ImportSpec:
		if n.Doc != nil {
			walk(state, v, n.Doc)
		}
		if n.Name != nil {
			walk(state, v, n.Name)
		}
		walk(state, v, n.Path)
		if n.Comment != nil {
			walk(state, v, n.Comment)
		}

	case *ast.ValueSpec:
		if n.Doc != nil {
			walk(state, v, n.Doc)
		}
		walkIdentList(state, v, n.Names)
		if n.Type != nil {
			walk(state, v, n.Type)
		}
		walkExprList(state, v, n.Values)
		if n.Comment != nil {
			walk(state, v, n.Comment)
		}

	case *ast.TypeSpec:
		if n.Doc != nil {
			walk(state, v, n.Doc)
		}
		walk(state, v, n.Name)
		if n.TypeParams != nil {
			walk(state, v, n.TypeParams)
		}
		walk(state, v, n.Type)
		if n.Comment != nil {
			walk(state, v, n.Comment)
		}

	case *ast.BadDecl:
		// nothing to do

	case *ast.GenDecl:
		if n.Doc != nil {
			walk(state, v, n.Doc)
		}
		for _, s := range n.Specs {
			walk(state, v, s)
		}

	case *ast.FuncDecl:
		if n.Doc != nil {
			walk(state, v, n.Doc)
		}
		if n.Recv != nil {
			walk(state, v, n.Recv)
		}
		walk(state, v, n.Name)
		walk(state, v, n.Type)
		if n.Body != nil {
			walk(state, v, n.Body)
		}

	// Files and packages
	case *ast.File:
		if n.Doc != nil {
			walk(state, v, n.Doc)
		}
		walk(state, v, n.Name)
		walkDeclList(state, v, n.Decls)
		// don't walk n.Comments - they have been
		// visited already through the individual
		// nodes

	case *ast.Package:
		for _, f := range n.Files {
			walk(state, v, f)
		}

	default:
		panic(fmt.Sprintf("ast.Walk: unexpected node type %T", n))
	}

	v.Visit(state, nil)
}

func walkIdentList(ctx *State, v Visitor, list []*ast.Ident) {
	for _, x := range list {
		walk(ctx, v, x)
	}
}

func walkExprList(ctx *State, v Visitor, list []ast.Expr) {
	for _, x := range list {
		walk(ctx, v, x)
	}
}

func walkStmtList(ctx *State, v Visitor, list []ast.Stmt) {
	for _, x := range list {
		walk(ctx, v, x)
	}
}

func walkDeclList(ctx *State, v Visitor, list []ast.Decl) {
	for _, x := range list {
		walk(ctx, v, x)
	}
}
