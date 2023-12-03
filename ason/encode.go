package ason

import (
	"go/ast"
	"go/token"
)

type SerConfig struct {
	// RefCounterEnable flag must be used when you carry out some manual
	// manipulations with the source AST tree. For example, you duplicate nodes,
	// which can create nodes that have the same references to the original object in memory.
	//
	// Use this flag when duplicating nodes containing many fields.
	RefCounterEnable bool

	// Standard Position structure contains fields such as `Filename`, `Offset`, `Line` and `Column`.
	// Usually, all this information is not required for, but only the
	// `Line` in the source code file and `Filename` is required.
	//
	// Use this flag when you do not need support for backward compatibility with the original AST
	// and you do not require the fields that the standard position structure contains
	PosCompress bool
}

// SerPassOptFn is a functional option type that allows us to configure the SerPass.
type SerPassOptFn func(*SerPass)

type SerPass struct {
	fset     *token.FileSet
	ref      map[any]any
	refcount uint
	conf     *SerConfig
}

func NewPass(fset *token.FileSet, options ...SerPassOptFn) *SerPass {
	pass := &SerPass{
		fset: fset,
		conf: new(SerConfig),
	}

	for _, opt := range options {
		opt(pass)
	}

	return pass
}

func WithRefCounter() SerPassOptFn {
	return func(pass *SerPass) {
		pass.ref = make(map[any]any)
		pass.conf.RefCounterEnable = true
	}
}

func WithPosCompression() SerPassOptFn {
	return func(pass *SerPass) {
		pass.conf.PosCompress = true
	}
}

type SerFn[I ast.Node, R Ason] func(*SerPass, I) R

func WithRefLookup[I ast.Node, R Ason](pass *SerPass, input I, ser SerFn[I, R]) R {
	if node, exists := pass.ref[input]; exists {
		return node.(R)
	}

	node := ser(pass, input)

	pass.refcount++
	pass.ref[input] = node

	return node
}

func ProcessList[I ast.Node, R Ason](pass *SerPass, inputList []I, ser SerFn[I, R]) []R {
	if inputList == nil {
		return nil
	}

	result := make([]R, len(inputList))
	for i := 0; i < len(inputList); i++ {
		result[i] = ser(pass, inputList[i])
	}

	return result
}

func ProcessNode(node ast.Node) Node {
	return Node{
		Type: ProcessNodeType(node),
	}
}

func ProcessNodeWithRef(node ast.Node, ref uint) Node {
	return Node{
		Ref:  ref,
		Type: ProcessNodeType(node),
	}
}

func SerializeFile(pass *SerPass, input *ast.File) *File {
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

func SerializePos(pass *SerPass, pos token.Pos) Pos {
	position := pass.fset.PositionFor(pos, false)

	if pass.conf.PosCompress {
		return &PosCompressed{
			Filename: position.Filename,
			Line:     position.Line,
		}
	}

	return &Position{
		Filename: position.Filename,
		Offset:   position.Offset,
		Line:     position.Line,
		Column:   position.Column,
	}
}

func ProcessPos(pass *SerPass, pos token.Pos) Pos {
	if pos != token.NoPos {
		return SerializePos(pass, pos)
	}

	return new(NoPos)
}

func ProcessLoc(pass *SerPass, start, end token.Pos) *Loc {
	loc := new(Loc)

	if start != token.NoPos {
		loc.Start = SerializePos(pass, start)
	}

	if end != token.NoPos {
		loc.End = SerializePos(pass, end)
	}

	return loc
}

func SerializeComment(pass *SerPass, input *ast.Comment) *Comment {
	return &Comment{
		Node:  ProcessNode(input),
		Slash: ProcessPos(pass, input.Slash),
		Text:  input.Text,
	}
}

func SerializeCommentGroup(pass *SerPass, input *ast.CommentGroup) *CommentGroup {
	return &CommentGroup{
		Node: ProcessNode(input),
		List: ProcessList[*ast.Comment, *Comment](pass, input.List, SerializeComment),
	}
}

func ProcessCommentGroup(pass *SerPass, group *ast.CommentGroup) *CommentGroup {
	if group != nil {
		return SerializeCommentGroup(pass, group)
	}

	return nil
}

func SerializeIdent(pass *SerPass, input *ast.Ident) *Ident {
	if !pass.conf.RefCounterEnable {
		return serializeIdent(pass, input)
	}

	return WithRefLookup[*ast.Ident, *Ident](pass, input, serializeIdent)
}

func serializeIdent(pass *SerPass, input *ast.Ident) *Ident {
	return &Ident{
		Loc:     ProcessLoc(pass, input.Pos(), input.End()),
		NamePos: ProcessPos(pass, input.NamePos),
		Name:    input.Name,
		Node:    ProcessNodeWithRef(input, pass.refcount),
	}

}

func SerializeBasicLit(pass *SerPass, input *ast.BasicLit) *BasicLit {
	if !pass.conf.RefCounterEnable {
		return serializeBasicLit(pass, input)
	}

	return WithRefLookup[*ast.BasicLit, *BasicLit](pass, input, serializeBasicLit)
}

func serializeBasicLit(pass *SerPass, input *ast.BasicLit) *BasicLit {
	return &BasicLit{
		Loc:      ProcessLoc(pass, input.Pos(), input.End()),
		ValuePos: ProcessPos(pass, input.ValuePos),
		Kind:     input.Kind.String(),
		Value:    input.Value,
		Node:     ProcessNodeWithRef(input, pass.refcount),
	}
}

func SerializeValueSpec(pass *SerPass, input *ast.ValueSpec) *ValueSpec {
	if !pass.conf.RefCounterEnable {
		return serializeValueSpec(pass, input)
	}

	return WithRefLookup[*ast.ValueSpec, *ValueSpec](pass, input, serializeValueSpec)
}

func serializeValueSpec(pass *SerPass, input *ast.ValueSpec) *ValueSpec {
	return &ValueSpec{
		Loc:    ProcessLoc(pass, input.Pos(), input.End()),
		Values: ProcessList[ast.Expr, Expr](pass, input.Values, ProcessExpr),
		Node:   ProcessNode(input),
	}
}

func SerializeGenDecl(pass *SerPass, input *ast.GenDecl) *GenDecl {
	if !pass.conf.RefCounterEnable {
		return serializeGenDecl(pass, input)
	}

	return WithRefLookup[*ast.GenDecl, *GenDecl](pass, input, serializeGenDecl)
}

func serializeGenDecl(pass *SerPass, input *ast.GenDecl) *GenDecl {
	return &GenDecl{
		Loc:      ProcessLoc(pass, input.Pos(), input.End()),
		TokenPos: ProcessPos(pass, input.TokPos),
		Lparen:   ProcessPos(pass, input.Lparen),
		Rparen:   ProcessPos(pass, input.Rparen),
		Tok:      input.Tok.String(),
		Specs:    ProcessList[ast.Spec, Spec](pass, input.Specs, ProcessSpec),
	}
}

func ProcessExpr(pass *SerPass, expr ast.Expr) Expr {
	switch e := expr.(type) {
	case *ast.BasicLit:
		return SerializeBasicLit(pass, e)
	default:
		return nil
	}
}

func ProcessSpec(pass *SerPass, spec ast.Spec) Spec {
	switch s := spec.(type) {
	case *ast.ValueSpec:
		return SerializeValueSpec(pass, s)
	default:
		return nil
	}
}

func ProcessDecl(pass *SerPass, decl ast.Decl) Decl {
	switch d := decl.(type) {
	case *ast.GenDecl:
		return SerializeGenDecl(pass, d)
	default:
		return nil
	}
}

const (
	NodeTypeInvalid      = "<invalid>"
	NodeTypeFile         = "File"
	NodeTypeComment      = "Comment"
	NodeTypeCommentGroup = "CommentGroup"
	NodeTypeIdent        = "Ident"
	NodeTypeBasicLit     = "BasicLit"
	NodeTypeValueSpec    = "ValueSpec"
	NodeTypeGenDecl      = "GenDecl"
)

func ProcessNodeType(node ast.Node) string {
	// n :=
	switch node.(type) {
	case *ast.File:
		return NodeTypeFile
	case *ast.Comment:
		return NodeTypeComment
	case *ast.CommentGroup:
		return NodeTypeCommentGroup
	case *ast.Ident:
		return NodeTypeIdent
	case *ast.BasicLit:
		return NodeTypeBasicLit
	case *ast.ValueSpec:
		return NodeTypeValueSpec
	case *ast.GenDecl:
		return NodeTypeGenDecl
	default:
		return NodeTypeInvalid
	}
}
