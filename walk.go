package compass

import (
	"context"
	"fmt"
	"go/ast"
)

func WalkWithContext(ctx context.Context, v Visitor, node ast.Node) {
	if v = v.VisitWithContext(ctx, node); v == nil {
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
			WalkWithContext(ctx, v, c)
		}

	case *ast.Field:
		if n.Doc != nil {
			WalkWithContext(ctx, v, n.Doc)
		}
		walkIdentList(ctx, v, n.Names)
		if n.Type != nil {
			WalkWithContext(ctx, v, n.Type)
		}
		if n.Tag != nil {
			WalkWithContext(ctx, v, n.Tag)
		}
		if n.Comment != nil {
			WalkWithContext(ctx, v, n.Comment)
		}

	case *ast.FieldList:
		for _, f := range n.List {
			WalkWithContext(ctx, v, f)
		}

	// Expressions
	case *ast.BadExpr, *ast.Ident, *ast.BasicLit:
		// nothing to do

	case *ast.Ellipsis:
		if n.Elt != nil {
			WalkWithContext(ctx, v, n.Elt)
		}

	case *ast.FuncLit:
		WalkWithContext(ctx, v, n.Type)
		WalkWithContext(ctx, v, n.Body)

	case *ast.CompositeLit:
		if n.Type != nil {
			WalkWithContext(ctx, v, n.Type)
		}
		walkExprList(ctx, v, n.Elts)

	case *ast.ParenExpr:
		WalkWithContext(ctx, v, n.X)

	case *ast.SelectorExpr:
		WalkWithContext(ctx, v, n.X)
		WalkWithContext(ctx, v, n.Sel)

	case *ast.IndexExpr:
		WalkWithContext(ctx, v, n.X)
		WalkWithContext(ctx, v, n.Index)

	case *ast.IndexListExpr:
		WalkWithContext(ctx, v, n.X)
		for _, index := range n.Indices {
			WalkWithContext(ctx, v, index)
		}

	case *ast.SliceExpr:
		WalkWithContext(ctx, v, n.X)
		if n.Low != nil {
			WalkWithContext(ctx, v, n.Low)
		}
		if n.High != nil {
			WalkWithContext(ctx, v, n.High)
		}
		if n.Max != nil {
			WalkWithContext(ctx, v, n.Max)
		}

	case *ast.TypeAssertExpr:
		WalkWithContext(ctx, v, n.X)
		if n.Type != nil {
			WalkWithContext(ctx, v, n.Type)
		}

	case *ast.CallExpr:
		WalkWithContext(ctx, v, n.Fun)
		walkExprList(ctx, v, n.Args)

	case *ast.StarExpr:
		WalkWithContext(ctx, v, n.X)

	case *ast.UnaryExpr:
		WalkWithContext(ctx, v, n.X)

	case *ast.BinaryExpr:
		WalkWithContext(ctx, v, n.X)
		WalkWithContext(ctx, v, n.Y)

	case *ast.KeyValueExpr:
		WalkWithContext(ctx, v, n.Key)
		WalkWithContext(ctx, v, n.Value)

	// Types
	case *ast.ArrayType:
		if n.Len != nil {
			WalkWithContext(ctx, v, n.Len)
		}
		WalkWithContext(ctx, v, n.Elt)

	case *ast.StructType:
		WalkWithContext(ctx, v, n.Fields)

	case *ast.FuncType:
		if n.TypeParams != nil {
			WalkWithContext(ctx, v, n.TypeParams)
		}
		if n.Params != nil {
			WalkWithContext(ctx, v, n.Params)
		}
		if n.Results != nil {
			WalkWithContext(ctx, v, n.Results)
		}

	case *ast.InterfaceType:
		WalkWithContext(ctx, v, n.Methods)

	case *ast.MapType:
		WalkWithContext(ctx, v, n.Key)
		WalkWithContext(ctx, v, n.Value)

	case *ast.ChanType:
		WalkWithContext(ctx, v, n.Value)

	// Statements
	case *ast.BadStmt:
		// nothing to do

	case *ast.DeclStmt:
		WalkWithContext(ctx, v, n.Decl)

	case *ast.EmptyStmt:
		// nothing to do

	case *ast.LabeledStmt:
		WalkWithContext(ctx, v, n.Label)
		WalkWithContext(ctx, v, n.Stmt)

	case *ast.ExprStmt:
		WalkWithContext(ctx, v, n.X)

	case *ast.SendStmt:
		WalkWithContext(ctx, v, n.Chan)
		WalkWithContext(ctx, v, n.Value)

	case *ast.IncDecStmt:
		WalkWithContext(ctx, v, n.X)

	case *ast.AssignStmt:
		walkExprList(ctx, v, n.Lhs)
		walkExprList(ctx, v, n.Rhs)

	case *ast.GoStmt:
		WalkWithContext(ctx, v, n.Call)

	case *ast.DeferStmt:
		WalkWithContext(ctx, v, n.Call)

	case *ast.ReturnStmt:
		walkExprList(ctx, v, n.Results)

	case *ast.BranchStmt:
		if n.Label != nil {
			WalkWithContext(ctx, v, n.Label)
		}

	case *ast.BlockStmt:
		walkStmtList(ctx, v, n.List)

	case *ast.IfStmt:
		if n.Init != nil {
			WalkWithContext(ctx, v, n.Init)
		}
		WalkWithContext(ctx, v, n.Cond)
		WalkWithContext(ctx, v, n.Body)
		if n.Else != nil {
			WalkWithContext(ctx, v, n.Else)
		}

	case *ast.CaseClause:
		walkExprList(ctx, v, n.List)
		walkStmtList(ctx, v, n.Body)

	case *ast.SwitchStmt:
		if n.Init != nil {
			WalkWithContext(ctx, v, n.Init)
		}
		if n.Tag != nil {
			WalkWithContext(ctx, v, n.Tag)
		}
		WalkWithContext(ctx, v, n.Body)

	case *ast.TypeSwitchStmt:
		if n.Init != nil {
			WalkWithContext(ctx, v, n.Init)
		}
		WalkWithContext(ctx, v, n.Assign)
		WalkWithContext(ctx, v, n.Body)

	case *ast.CommClause:
		if n.Comm != nil {
			WalkWithContext(ctx, v, n.Comm)
		}
		walkStmtList(ctx, v, n.Body)

	case *ast.SelectStmt:
		WalkWithContext(ctx, v, n.Body)

	case *ast.ForStmt:
		if n.Init != nil {
			WalkWithContext(ctx, v, n.Init)
		}
		if n.Cond != nil {
			WalkWithContext(ctx, v, n.Cond)
		}
		if n.Post != nil {
			WalkWithContext(ctx, v, n.Post)
		}
		WalkWithContext(ctx, v, n.Body)

	case *ast.RangeStmt:
		if n.Key != nil {
			WalkWithContext(ctx, v, n.Key)
		}
		if n.Value != nil {
			WalkWithContext(ctx, v, n.Value)
		}
		WalkWithContext(ctx, v, n.X)
		WalkWithContext(ctx, v, n.Body)

	// Declarations
	case *ast.ImportSpec:
		if n.Doc != nil {
			WalkWithContext(ctx, v, n.Doc)
		}
		if n.Name != nil {
			WalkWithContext(ctx, v, n.Name)
		}
		WalkWithContext(ctx, v, n.Path)
		if n.Comment != nil {
			WalkWithContext(ctx, v, n.Comment)
		}

	case *ast.ValueSpec:
		if n.Doc != nil {
			WalkWithContext(ctx, v, n.Doc)
		}
		walkIdentList(ctx, v, n.Names)
		if n.Type != nil {
			WalkWithContext(ctx, v, n.Type)
		}
		walkExprList(ctx, v, n.Values)
		if n.Comment != nil {
			WalkWithContext(ctx, v, n.Comment)
		}

	case *ast.TypeSpec:
		if n.Doc != nil {
			WalkWithContext(ctx, v, n.Doc)
		}
		WalkWithContext(ctx, v, n.Name)
		if n.TypeParams != nil {
			WalkWithContext(ctx, v, n.TypeParams)
		}
		WalkWithContext(ctx, v, n.Type)
		if n.Comment != nil {
			WalkWithContext(ctx, v, n.Comment)
		}

	case *ast.BadDecl:
		// nothing to do

	case *ast.GenDecl:
		if n.Doc != nil {
			WalkWithContext(ctx, v, n.Doc)
		}
		for _, s := range n.Specs {
			WalkWithContext(ctx, v, s)
		}

	case *ast.FuncDecl:
		if n.Doc != nil {
			WalkWithContext(ctx, v, n.Doc)
		}
		if n.Recv != nil {
			WalkWithContext(ctx, v, n.Recv)
		}
		WalkWithContext(ctx, v, n.Name)
		WalkWithContext(ctx, v, n.Type)
		if n.Body != nil {
			WalkWithContext(ctx, v, n.Body)
		}

	// Files and packages
	case *ast.File:
		if n.Doc != nil {
			WalkWithContext(ctx, v, n.Doc)
		}
		WalkWithContext(ctx, v, n.Name)
		walkDeclList(ctx, v, n.Decls)
		// don't walk n.Comments - they have been
		// visited already through the individual
		// nodes

	case *ast.Package:
		for _, f := range n.Files {
			WalkWithContext(ctx, v, f)
		}

	default:
		panic(fmt.Sprintf("ast.Walk: unexpected node type %T", n))
	}

	v.VisitWithContext(ctx, nil)
}

func walkIdentList(ctx context.Context, v Visitor, list []*ast.Ident) {
	for _, x := range list {
		WalkWithContext(ctx, v, x)
	}
}

func walkExprList(ctx context.Context, v Visitor, list []ast.Expr) {
	for _, x := range list {
		WalkWithContext(ctx, v, x)
	}
}

func walkStmtList(ctx context.Context, v Visitor, list []ast.Stmt) {
	for _, x := range list {
		WalkWithContext(ctx, v, x)
	}
}

func walkDeclList(ctx context.Context, v Visitor, list []ast.Decl) {
	for _, x := range list {
		WalkWithContext(ctx, v, x)
	}
}
