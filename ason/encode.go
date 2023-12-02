package ason

import (
	"go/ast"
	"go/token"
)

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

type SerdeFunc[I ast.Node, R Ason] func(*Pass, I) R

func WithRefLookup[I ast.Node, R Ason](pass *Pass, input I, serde SerdeFunc[I, R]) R {
	if node, exists := pass.ref[input]; exists {
		return node.(R)
	}
	pass.refcount++
	node := serde(pass, input)
	pass.ref[input] = node

	return node
}

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
		Decls:    ProcessDeclList(pass, input.Decls),
		Doc:      ProcessCommentGroup(pass, input.Doc),
		Package:  ProcessPos(pass, input.Package),
		Loc:      ProcessLoc(pass, input.FileStart, input.FileEnd),
		Comments: ProcessCommentGroupList(pass, input.Comments),
		Node:     ProcessNode(input),
	}
}

func SerializePos(pass *Pass, pos token.Pos) *Position {
	position := pass.fset.PositionFor(pos, false)

	return &Position{
		Filename: position.Filename,
		Offset:   position.Offset,
		Line:     position.Line,
		Column:   position.Column,
	}
}

func ProcessPos(pass *Pass, pos token.Pos) *Position {
	if pos != token.NoPos {
		return SerializePos(pass, pos)
	}

	// TODO: when NoPos return should be 0, now null
	return nil
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

func SerializeCommentList(pass *Pass, inputList []*ast.Comment) []*Comment {
	result := make([]*Comment, len(inputList))

	for i := 0; i < len(inputList); i++ {
		result[i] = SerializeComment(pass, inputList[i])
	}

	return result
}

func ProcessCommentList(pass *Pass, list []*ast.Comment) []*Comment {
	if list != nil {
		return SerializeCommentList(pass, list)
	}

	return nil
}

func SerializeCommentGroup(pass *Pass, input *ast.CommentGroup) *CommentGroup {
	return &CommentGroup{
		Node: ProcessNode(input),
		List: ProcessCommentList(pass, input.List),
	}
}

func ProcessCommentGroup(pass *Pass, group *ast.CommentGroup) *CommentGroup {
	if group != nil {
		return SerializeCommentGroup(pass, group)
	}

	return nil
}

func SerializeCommentGroupList(pass *Pass, inputList []*ast.CommentGroup) []*CommentGroup {
	result := make([]*CommentGroup, len(inputList))

	for i := 0; i < len(inputList); i++ {
		result[i] = ProcessCommentGroup(pass, inputList[i])
	}

	return result
}

func ProcessCommentGroupList(pass *Pass, list []*ast.CommentGroup) []*CommentGroup {
	if list != nil {
		return SerializeCommentGroupList(pass, list)
	}

	return nil
}

func SerializeIdent(pass *Pass, input *ast.Ident) *Ident {
	if node, exists := pass.ref[input]; exists {
		return node.(*Ident)
	}
	pass.refcount++

	node := &Ident{
		Loc:     ProcessLoc(pass, input.Pos(), input.End()),
		NamePos: ProcessPos(pass, input.NamePos),
		Name:    input.Name,
		Node:    ProcessNodeWithRef(input, pass.refcount),
	}

	pass.ref[input] = node

	return node
}

func SerializeBasicLit(pass *Pass, input *ast.BasicLit) *BasicLit {
	if node, exists := pass.ref[input]; exists {
		return node.(*BasicLit)
	}

	pass.refcount++

	node := &BasicLit{
		Loc:      ProcessLoc(pass, input.Pos(), input.End()),
		ValuePos: ProcessPos(pass, input.ValuePos),
		Kind:     input.Kind.String(),
		Value:    input.Value,
		Node:     ProcessNodeWithRef(input, pass.refcount),
	}

	pass.ref[input] = node

	return node
}

func SerializeValueSpec(pass *Pass, input *ast.ValueSpec) *ValueSpec {
	if node, exists := pass.ref[input]; exists {
		return node.(*ValueSpec)
	}

	pass.refcount++

	node := &ValueSpec{
		Loc:    ProcessLoc(pass, input.Pos(), input.End()),
		Values: ProcessExprList(pass, input.Values),
		Node:   ProcessNode(input),
	}

	pass.ref[input] = node

	return node
}

func ProcessExpr(pass *Pass, expr ast.Expr) Expr {
	switch e := expr.(type) {
	case *ast.BasicLit:
		return SerializeBasicLit(pass, e)
	default:
		return nil
	}
}

func ProcessExprList(pass *Pass, exprList []ast.Expr) []Expr {
	if exprList == nil {
		return nil
	}

	result := make([]Expr, len(exprList))

	for i := 0; i < len(exprList); i++ {
		result[i] = ProcessExpr(pass, exprList[i])
	}

	return result
}

func SerializeGenDecl(pass *Pass, input *ast.GenDecl) *GenDecl {
	return &GenDecl{
		Loc:      ProcessLoc(pass, input.Pos(), input.End()),
		TokenPos: ProcessPos(pass, input.TokPos),
		Lparen:   ProcessPos(pass, input.Lparen),
		Rparen:   ProcessPos(pass, input.Rparen),
		Tok:      input.Tok.String(),
		Specs:    ProcessSpecList(pass, input.Specs),
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

func ProcessSpecList(pass *Pass, specList []ast.Spec) []Spec {
	if specList == nil {
		return nil
	}

	result := make([]Spec, len(specList))

	for i := 0; i < len(specList); i++ {
		result[i] = ProcessSpec(pass, specList[i])
	}

	return result
}

func ProcessDeclList(pass *Pass, declList []ast.Decl) []Decl {
	if declList == nil {
		return nil
	}

	result := make([]Decl, len(declList))

	for i := 0; i < len(declList); i++ {
		result[i] = ProcessDecl(pass, declList[i])
	}

	return result
}

func ProcessDecl(pass *Pass, decl ast.Decl) Decl {
	switch d := decl.(type) {
	case *ast.GenDecl:
		return SerializeGenDecl(pass, d)
	default:
		return nil
	}
}

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
