package compass

import (
	"fmt"
	"go/ast"

	"github.com/4rchr4y/go-compass/state"
)

func Walk(state *state.State, v Visitor, node ast.Node) {
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
			Walk(state, v, c)
		}

	case *ast.Field:
		if n.Doc != nil {
			Walk(state, v, n.Doc)
		}
		walkIdentList(state, v, n.Names)
		if n.Type != nil {
			Walk(state, v, n.Type)
		}
		if n.Tag != nil {
			Walk(state, v, n.Tag)
		}
		if n.Comment != nil {
			Walk(state, v, n.Comment)
		}

	case *ast.FieldList:
		for _, f := range n.List {
			Walk(state, v, f)
		}

	// Expressions
	case *ast.BadExpr, *ast.Ident, *ast.BasicLit:
		// nothing to do

	case *ast.Ellipsis:
		if n.Elt != nil {
			Walk(state, v, n.Elt)
		}

	case *ast.FuncLit:
		Walk(state, v, n.Type)
		Walk(state, v, n.Body)

	case *ast.CompositeLit:
		if n.Type != nil {
			Walk(state, v, n.Type)
		}
		walkExprList(state, v, n.Elts)

	case *ast.ParenExpr:
		Walk(state, v, n.X)

	case *ast.SelectorExpr:
		Walk(state, v, n.X)
		Walk(state, v, n.Sel)

	case *ast.IndexExpr:
		Walk(state, v, n.X)
		Walk(state, v, n.Index)

	case *ast.IndexListExpr:
		Walk(state, v, n.X)
		for _, index := range n.Indices {
			Walk(state, v, index)
		}

	case *ast.SliceExpr:
		Walk(state, v, n.X)
		if n.Low != nil {
			Walk(state, v, n.Low)
		}
		if n.High != nil {
			Walk(state, v, n.High)
		}
		if n.Max != nil {
			Walk(state, v, n.Max)
		}

	case *ast.TypeAssertExpr:
		Walk(state, v, n.X)
		if n.Type != nil {
			Walk(state, v, n.Type)
		}

	case *ast.CallExpr:
		Walk(state, v, n.Fun)
		walkExprList(state, v, n.Args)

	case *ast.StarExpr:
		Walk(state, v, n.X)

	case *ast.UnaryExpr:
		Walk(state, v, n.X)

	case *ast.BinaryExpr:
		Walk(state, v, n.X)
		Walk(state, v, n.Y)

	case *ast.KeyValueExpr:
		Walk(state, v, n.Key)
		Walk(state, v, n.Value)

	// Types
	case *ast.ArrayType:
		if n.Len != nil {
			Walk(state, v, n.Len)
		}
		Walk(state, v, n.Elt)

	case *ast.StructType:
		Walk(state, v, n.Fields)

	case *ast.FuncType:
		if n.TypeParams != nil {
			Walk(state, v, n.TypeParams)
		}
		if n.Params != nil {
			Walk(state, v, n.Params)
		}
		if n.Results != nil {
			Walk(state, v, n.Results)
		}

	case *ast.InterfaceType:
		Walk(state, v, n.Methods)

	case *ast.MapType:
		Walk(state, v, n.Key)
		Walk(state, v, n.Value)

	case *ast.ChanType:
		Walk(state, v, n.Value)

	// Statements
	case *ast.BadStmt:
		// nothing to do

	case *ast.DeclStmt:
		Walk(state, v, n.Decl)

	case *ast.EmptyStmt:
		// nothing to do

	case *ast.LabeledStmt:
		Walk(state, v, n.Label)
		Walk(state, v, n.Stmt)

	case *ast.ExprStmt:
		Walk(state, v, n.X)

	case *ast.SendStmt:
		Walk(state, v, n.Chan)
		Walk(state, v, n.Value)

	case *ast.IncDecStmt:
		Walk(state, v, n.X)

	case *ast.AssignStmt:
		walkExprList(state, v, n.Lhs)
		walkExprList(state, v, n.Rhs)

	case *ast.GoStmt:
		Walk(state, v, n.Call)

	case *ast.DeferStmt:
		Walk(state, v, n.Call)

	case *ast.ReturnStmt:
		walkExprList(state, v, n.Results)

	case *ast.BranchStmt:
		if n.Label != nil {
			Walk(state, v, n.Label)
		}

	case *ast.BlockStmt:
		walkStmtList(state, v, n.List)

	case *ast.IfStmt:
		if n.Init != nil {
			Walk(state, v, n.Init)
		}
		Walk(state, v, n.Cond)
		Walk(state, v, n.Body)
		if n.Else != nil {
			Walk(state, v, n.Else)
		}

	case *ast.CaseClause:
		walkExprList(state, v, n.List)
		walkStmtList(state, v, n.Body)

	case *ast.SwitchStmt:
		if n.Init != nil {
			Walk(state, v, n.Init)
		}
		if n.Tag != nil {
			Walk(state, v, n.Tag)
		}
		Walk(state, v, n.Body)

	case *ast.TypeSwitchStmt:
		if n.Init != nil {
			Walk(state, v, n.Init)
		}
		Walk(state, v, n.Assign)
		Walk(state, v, n.Body)

	case *ast.CommClause:
		if n.Comm != nil {
			Walk(state, v, n.Comm)
		}
		walkStmtList(state, v, n.Body)

	case *ast.SelectStmt:
		Walk(state, v, n.Body)

	case *ast.ForStmt:
		if n.Init != nil {
			Walk(state, v, n.Init)
		}
		if n.Cond != nil {
			Walk(state, v, n.Cond)
		}
		if n.Post != nil {
			Walk(state, v, n.Post)
		}
		Walk(state, v, n.Body)

	case *ast.RangeStmt:
		if n.Key != nil {
			Walk(state, v, n.Key)
		}
		if n.Value != nil {
			Walk(state, v, n.Value)
		}
		Walk(state, v, n.X)
		Walk(state, v, n.Body)

	// Declarations
	case *ast.ImportSpec:
		if n.Doc != nil {
			Walk(state, v, n.Doc)
		}
		if n.Name != nil {
			Walk(state, v, n.Name)
		}
		Walk(state, v, n.Path)
		if n.Comment != nil {
			Walk(state, v, n.Comment)
		}

	case *ast.ValueSpec:
		if n.Doc != nil {
			Walk(state, v, n.Doc)
		}
		walkIdentList(state, v, n.Names)
		if n.Type != nil {
			Walk(state, v, n.Type)
		}
		walkExprList(state, v, n.Values)
		if n.Comment != nil {
			Walk(state, v, n.Comment)
		}

	case *ast.TypeSpec:
		if n.Doc != nil {
			Walk(state, v, n.Doc)
		}
		Walk(state, v, n.Name)
		if n.TypeParams != nil {
			Walk(state, v, n.TypeParams)
		}
		Walk(state, v, n.Type)
		if n.Comment != nil {
			Walk(state, v, n.Comment)
		}

	case *ast.BadDecl:
		// nothing to do

	case *ast.GenDecl:
		if n.Doc != nil {
			Walk(state, v, n.Doc)
		}
		for _, s := range n.Specs {
			Walk(state, v, s)
		}

	case *ast.FuncDecl:
		if n.Doc != nil {
			Walk(state, v, n.Doc)
		}
		if n.Recv != nil {
			Walk(state, v, n.Recv)
		}
		Walk(state, v, n.Name)
		Walk(state, v, n.Type)
		if n.Body != nil {
			Walk(state, v, n.Body)
		}

	// Files and packages
	case *ast.File:
		if n.Doc != nil {
			Walk(state, v, n.Doc)
		}
		Walk(state, v, n.Name)
		walkDeclList(state, v, n.Decls)
		// don't walk n.Comments - they have been
		// visited already through the individual
		// nodes

	case *ast.Package:
		for _, f := range n.Files {
			Walk(state, v, f)
		}

	default:
		panic(fmt.Sprintf("ast.Walk: unexpected node type %T", n))
	}

	v.Visit(state, nil)
}

func walkIdentList(ctx *state.State, v Visitor, list []*ast.Ident) {
	for _, x := range list {
		Walk(ctx, v, x)
	}
}

func walkExprList(ctx *state.State, v Visitor, list []ast.Expr) {
	for _, x := range list {
		Walk(ctx, v, x)
	}
}

func walkStmtList(ctx *state.State, v Visitor, list []ast.Stmt) {
	for _, x := range list {
		Walk(ctx, v, x)
	}
}

func walkDeclList(ctx *state.State, v Visitor, list []ast.Decl) {
	for _, x := range list {
		Walk(ctx, v, x)
	}
}
