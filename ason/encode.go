package ason

import (
	"go/ast"
	"go/token"
)

type Pass struct {
	fset     *token.FileSet
	ref      map[any]any
	refcount uint
}

func NewPass(fset *token.FileSet) *Pass {
	return &Pass{
		fset: fset,
		ref:  make(map[any]any),
	}
}

type SerdeFunc[I ast.Node, R Ason] func(*Pass, I) R

func WithRefLookup[I ast.Node, R Ason](pass *Pass, input I, serde SerdeFunc[I, R]) R {
	if node, exists := pass.ref[input]; exists {
		return node.(R)
	}

	node := serde(pass, input)

	pass.refcount++
	pass.ref[input] = node

	return node
}

func ProcessList[I ast.Node, R Ason](pass *Pass, inputList []I, serde SerdeFunc[I, R]) []R {
	if inputList == nil {
		return nil
	}

	result := make([]R, len(inputList))
	for i := 0; i < len(inputList); i++ {
		result[i] = serde(pass, inputList[i])
	}

	return result
}

func ProcessNode(node ast.Node) Node {
	return Node{
		Type: defineNodeType(node),
	}
}

func ProcessNodeWithRef(node ast.Node, ref uint) Node {
	return Node{
		Ref:  ref,
		Type: defineNodeType(node),
	}
}

func SerializeFile(pass *Pass, input *ast.File) *File {
	return &File{
		Name:     SerializeIdent(pass, input.Name),
		Decls:    ProcessList[ast.Decl, Decl](pass, input.Decls, ProcessDecl),
		Doc:      ProcessCommentGroup(pass, input.Doc),
		Package:  ProcessPos(pass, input.Package),
		Loc:      ProcessLoc(pass, input.FileStart, input.FileEnd),
		Comments: ProcessList[*ast.CommentGroup, *CommentGroup](pass, input.Comments, ProcessCommentGroup),
		Node:     ProcessNode(input),
	}
}

func SerializePos(pass *Pass, pos token.Pos) Pos {
	position := pass.fset.PositionFor(pos, false)

	return &Position{
		Filename: position.Filename,
		Offset:   position.Offset,
		Line:     position.Line,
		Column:   position.Column,
	}
}

func ProcessPos(pass *Pass, pos token.Pos) Pos {
	if pos != token.NoPos {
		return SerializePos(pass, pos)
	}

	return new(NoPos)
}

func ProcessLoc(pass *Pass, start, end token.Pos) *Loc {
	loc := new(Loc)

	if start != token.NoPos {
		loc.Start = SerializePos(pass, start)
	}

	if end != token.NoPos {
		loc.End = SerializePos(pass, end)
	}

	return loc
}

func SerializeComment(pass *Pass, input *ast.Comment) *Comment {
	return &Comment{
		Node:  ProcessNode(input),
		Slash: ProcessPos(pass, input.Slash),
		Text:  input.Text,
	}
}

func SerializeCommentGroup(pass *Pass, input *ast.CommentGroup) *CommentGroup {
	return &CommentGroup{
		Node: ProcessNode(input),
		List: ProcessList[*ast.Comment, *Comment](pass, input.List, SerializeComment),
	}
}

func ProcessCommentGroup(pass *Pass, group *ast.CommentGroup) *CommentGroup {
	if group != nil {
		return SerializeCommentGroup(pass, group)
	}

	return nil
}

func SerializeIdent(pass *Pass, input *ast.Ident) *Ident {
	return WithRefLookup[*ast.Ident, *Ident](pass, input, serializeIdent)
}

func serializeIdent(pass *Pass, input *ast.Ident) *Ident {
	return &Ident{
		Loc:     ProcessLoc(pass, input.Pos(), input.End()),
		NamePos: ProcessPos(pass, input.NamePos),
		Name:    input.Name,
		Node:    ProcessNodeWithRef(input, pass.refcount),
	}

}

func SerializeBasicLit(pass *Pass, input *ast.BasicLit) *BasicLit {
	return WithRefLookup[*ast.BasicLit, *BasicLit](pass, input, serializeBasicLit)
}

func serializeBasicLit(pass *Pass, input *ast.BasicLit) *BasicLit {
	return &BasicLit{
		Loc:      ProcessLoc(pass, input.Pos(), input.End()),
		ValuePos: ProcessPos(pass, input.ValuePos),
		Kind:     input.Kind.String(),
		Value:    input.Value,
		Node:     ProcessNodeWithRef(input, pass.refcount),
	}
}

func SerializeValueSpec(pass *Pass, input *ast.ValueSpec) *ValueSpec {
	return WithRefLookup[*ast.ValueSpec, *ValueSpec](pass, input, serializeValueSpec)
}

func serializeValueSpec(pass *Pass, input *ast.ValueSpec) *ValueSpec {
	return &ValueSpec{
		Loc:    ProcessLoc(pass, input.Pos(), input.End()),
		Values: ProcessList[ast.Expr, Expr](pass, input.Values, ProcessExpr),
		Node:   ProcessNode(input),
	}
}

func ProcessExpr(pass *Pass, expr ast.Expr) Expr {
	switch e := expr.(type) {
	case *ast.BasicLit:
		return SerializeBasicLit(pass, e)
	default:
		return nil
	}
}

func SerializeGenDecl(pass *Pass, input *ast.GenDecl) *GenDecl {
	return &GenDecl{
		Loc:      ProcessLoc(pass, input.Pos(), input.End()),
		TokenPos: ProcessPos(pass, input.TokPos),
		Lparen:   ProcessPos(pass, input.Lparen),
		Rparen:   ProcessPos(pass, input.Rparen),
		Tok:      input.Tok.String(),
		Specs:    ProcessList[ast.Spec, Spec](pass, input.Specs, ProcessSpec),
	}
}

func ProcessSpec(pass *Pass, spec ast.Spec) Spec {
	switch s := spec.(type) {
	case *ast.ValueSpec:
		return SerializeValueSpec(pass, s)
	default:
		return nil
	}
}

func ProcessDecl(pass *Pass, decl ast.Decl) Decl {
	switch d := decl.(type) {
	case *ast.GenDecl:
		return SerializeGenDecl(pass, d)
	default:
		return nil
	}
}
