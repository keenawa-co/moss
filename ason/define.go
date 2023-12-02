package ason

import "go/ast"

const (
	TypeInvalid      = "<invalid>"
	TypeFile         = "File"
	TypeComment      = "Comment"
	TypeCommentGroup = "CommentGroup"
	TypeIdent        = "Ident"
	TypeBasicLit     = "BasicLit"
	TypeValueSpec    = "ValueSpec"
	TypeGenDecl      = "GenDecl"
)

func defineNodeType(node ast.Node) string {
	// n :=
	switch node.(type) {
	case *ast.File:
		return TypeFile
	case *ast.Comment:
		return TypeComment
	case *ast.CommentGroup:
		return TypeCommentGroup
	case *ast.Ident:
		return TypeIdent
	case *ast.BasicLit:
		return TypeBasicLit
	case *ast.ValueSpec:
		return TypeValueSpec
	case *ast.GenDecl:
		return TypeGenDecl
	default:
		return TypeInvalid
	}
}
