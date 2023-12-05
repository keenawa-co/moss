package ason

import (
	"go/ast"
	"go/token"
	"os"
)

// TODO List
//
// *ast.FieldList
// *ast.FuncLit
// *ast.CompositeLit
// *ast.ParenExpr
// *ast.SelectorExpr
// *ast.IndexExpr
// *ast.IndexListExpr
// *ast.SliceExpr
// *ast.TypeAssertExpr
// *ast.CallExpr:
// *ast.StarExpr:
// *ast.UnaryExpr:
// *ast.BinaryExpr
// *ast.KeyValueExpr
// *ast.ArrayType
// *ast.StructType
// *ast.FuncType
// *ast.InterfaceType
// *ast.MapType
// *ast.ChanType
// *ast.BadStmt
// *ast.DeclStmt
// *ast.EmptyStmt
// *ast.LabeledStmt
// *ast.ExprStmt
// *ast.SendStmt
// *ast.IncDecStmt
// *ast.AssignStmt
// *ast.GoStmt
// *ast.DeferStmt
// *ast.ReturnStmt
// *ast.BranchStmt
// *ast.BlockStmt
// *ast.IfStmt
// *ast.CaseClause
// *ast.SwitchStmt
// *ast.TypeSwitchStmt
// *ast.CommClause
// *ast.SelectStmt
// *ast.ForStmt
// *ast.RangeStmt
// *ast.ImportSpec
// *ast.TypeSpec
// *ast.BadDecl
// *ast.GenDecl
// *ast.FuncDecl
// *ast.File
// *ast.Package

type SerializationConf int

const (
	// CACHE_REF flag must be used when you carry out some manual manipulations with the source AST tree.
	// For example, you duplicate nodes, which can create nodes that have the same references to the original object in memory.
	//
	// Use this flag when duplicating nodes containing many fields.
	CACHE_REF SerializationConf = iota

	// POS_COMPRESS flag must be used when you not need support for backward compatibility with the original AST
	// and you do not require the fields that the standard position structure contains
	//
	// Standard *Position structure contains fields such as `Filename`, `Offset`, `Line` and `Column`.
	// Usually, all this information is not required for, but only the
	// `Line` in the source code file and `Filename` is required.
	POS_COMPRESS
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

type SerPass struct {
	fset     *token.FileSet
	readFile func(string) ([]byte, error)
	refCache map[ast.Node]*weakRef
	refCount uint
	conf     *SerConfig
}

// SerPassOptFn is a functional option type that allows us to configure the SerPass.
type SerPassOptFn func(*SerPass)

func NewSerPass(fset *token.FileSet, options ...SerPassOptFn) *SerPass {
	pass := &SerPass{
		fset:     fset,
		readFile: os.ReadFile,
		conf:     new(SerConfig),
	}

	for _, opt := range options {
		opt(pass)
	}

	return pass
}

func WithReadFileFn(fn func(string) ([]byte, error)) SerPassOptFn {
	return func(pass *SerPass) {
		pass.readFile = fn
	}
}

func WithRefCounter() SerPassOptFn {
	return func(pass *SerPass) {
		pass.refCache = make(map[ast.Node]*weakRef)
		pass.conf.RefCounterEnable = true
	}
}

func WithPosCompression() SerPassOptFn {
	return func(pass *SerPass) {
		pass.conf.PosCompress = true
	}
}

type SerFn[I ast.Node, R Ason] func(*SerPass, I) R

func WithRefLookup[I ast.Node, R Ason](pass *SerPass, input I, serFn SerFn[I, R]) R {
	if weakRef, exists := pass.refCache[input]; exists {
		return weakRef.Load().(R)
	}

	node := serFn(pass, input)
	pass.refCache[input] = NewWeakRef(&node)
	pass.refCount++

	return node
}

func SerProcessList[I ast.Node, R Ason](pass *SerPass, inputList []I, serFn SerFn[I, R]) []R {
	if inputList == nil {
		return nil
	}

	result := make([]R, len(inputList))
	for i := 0; i < len(inputList); i++ {
		result[i] = serFn(pass, inputList[i])
	}

	return result
}

func SerializeFile(pass *SerPass, input *ast.File) *File {
	return &File{
		Name:    SerializeIdent(pass, input.Name),
		Decls:   SerProcessList[ast.Decl, Decl](pass, input.Decls, SerProcessDecl),
		Doc:     SerializeCommentGroup(pass, input.Doc),
		Package: SerializePos(pass, input.Package),
		Loc: &Loc{
			Start: SerializePos(pass, input.FileStart),
			End:   SerializePos(pass, input.FileEnd),
		},
		Size:      calcFileSize(pass, input),
		Comments:  SerProcessList[*ast.CommentGroup, *CommentGroup](pass, input.Comments, SerializeCommentGroup),
		GoVersion: input.GoVersion,
		Node:      Node{Type: NodeTypeFile, Ref: pass.refCount},
	}
}

func calcFileSize(pass *SerPass, input *ast.File) int {
	position := pass.fset.PositionFor(input.Name.NamePos, false)
	content, err := pass.readFile(position.Filename)
	if err != nil {
		return 1<<_GOARCH() - 2
	}

	return len(content)
}

func SerializePos(pass *SerPass, pos token.Pos) Pos {
	if pos == token.NoPos {
		return new(NoPos)
	}

	position := pass.fset.PositionFor(pos, false)

	if pass.conf.PosCompress {
		return serializePosCompress(pass, &position)
	}

	return serializePosition(pass, &position)
}

func serializePosition(pass *SerPass, pos *token.Position) *Position {
	return &Position{
		Filename: pos.Filename,
		Offset:   pos.Offset,
		Line:     pos.Line,
		Column:   pos.Column,
	}
}

func serializePosCompress(pass *SerPass, pos *token.Position) *PosCompressed {
	return &PosCompressed{
		Filename: pos.Filename,
		Line:     pos.Line,
	}
}

// TODO: add tests
func SerializeComment(pass *SerPass, input *ast.Comment) *Comment {
	return &Comment{
		Slash: SerializePos(pass, input.Slash),
		Text:  input.Text,
		Node:  Node{Type: NodeTypeComment, Ref: pass.refCount},
	}
}

// TODO: add tests
func SerializeCommentGroup(pass *SerPass, group *ast.CommentGroup) *CommentGroup {
	if group != nil {
		return serializeCommentGroup(pass, group)
	}

	return nil
}

// TODO: add tests
func serializeCommentGroup(pass *SerPass, input *ast.CommentGroup) *CommentGroup {
	return &CommentGroup{
		List: SerProcessList[*ast.Comment, *Comment](pass, input.List, SerializeComment),
		Node: Node{Type: NodeTypeCommentGroup, Ref: pass.refCount},
	}
}

func SerializeIdent(pass *SerPass, input *ast.Ident) *Ident {
	if input == nil {
		return nil
	}

	if !pass.conf.RefCounterEnable {
		return serializeIdent(pass, input)
	}

	return WithRefLookup[*ast.Ident, *Ident](pass, input, serializeIdent)
}

func serializeIdent(pass *SerPass, input *ast.Ident) *Ident {
	return &Ident{
		Loc: &Loc{
			Start: SerializePos(pass, input.Pos()),
			End:   SerializePos(pass, input.End()),
		},
		NamePos: SerializePos(pass, input.NamePos),
		Name:    input.Name,
		Node:    Node{Type: NodeTypeIdent, Ref: pass.refCount},
	}
}

//
// Everything before this line already have some tests
// <---- TESTED CURSOR
//

func SerializeBasicLit(pass *SerPass, input *ast.BasicLit) *BasicLit {
	if !pass.conf.RefCounterEnable {
		return serializeBasicLit(pass, input)
	}

	return WithRefLookup[*ast.BasicLit, *BasicLit](pass, input, serializeBasicLit)
}

func serializeBasicLit(pass *SerPass, input *ast.BasicLit) *BasicLit {
	return &BasicLit{
		Loc: &Loc{
			Start: SerializePos(pass, input.Pos()),
			End:   SerializePos(pass, input.End()),
		},
		ValuePos: SerializePos(pass, input.ValuePos),
		Kind:     input.Kind.String(),
		Value:    input.Value,
		Node:     Node{Type: NodeTypeBasicLit, Ref: pass.refCount},
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
		Loc: &Loc{
			Start: SerializePos(pass, input.Pos()),
			End:   SerializePos(pass, input.End()),
		},
		Names:  SerProcessList[*ast.Ident, *Ident](pass, input.Names, SerializeIdent),
		Values: SerProcessList[ast.Expr, Expr](pass, input.Values, SerProcessExpr),
		Node:   Node{Type: NodeTypeValueSpec, Ref: pass.refCount},
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
		Loc: &Loc{
			Start: SerializePos(pass, input.Pos()),
			End:   SerializePos(pass, input.End()),
		},
		TokenPos: SerializePos(pass, input.TokPos),
		Lparen:   SerializePos(pass, input.Lparen),
		Rparen:   SerializePos(pass, input.Rparen),
		Tok:      input.Tok.String(),
		Specs:    SerProcessList[ast.Spec, Spec](pass, input.Specs, SerProcessSpec),
		Node:     Node{Type: NodeTypeGenDecl, Ref: pass.refCount},
	}
}

func serializeField(pass *SerPass, input *ast.Field) *Field {
	return &Field{
		Loc: &Loc{
			Start: SerializePos(pass, input.Pos()),
			End:   SerializePos(pass, input.End()),
		},
		Names:   SerProcessList[*ast.Ident, *Ident](pass, input.Names, SerializeIdent),
		Type:    SerProcessExpr(pass, input.Type),
		Tag:     serializeBasicLit(pass, input.Tag),
		Comment: SerializeCommentGroup(pass, input.Comment),
		Node:    Node{Type: NodeTypeField, Ref: pass.refCount},
	}
}

func serializeBadExpr(pass *SerPass, input *ast.BadExpr) *BadExpr {
	return &BadExpr{
		Loc: &Loc{
			Start: SerializePos(pass, input.From),
			End:   SerializePos(pass, input.To),
		},
		Node: Node{Type: NodeTypeBadExpr, Ref: pass.refCount},
	}
}

func serializeCompositeLit(pass *SerPass, input *ast.CompositeLit) *CompositeLit {
	return &CompositeLit{
		Loc: &Loc{
			Start: SerializePos(pass, input.Pos()),
			End:   SerializePos(pass, input.End()),
		},
	}
}

func serializeEllipsis(pass *SerPass, input *ast.Ellipsis) *Ellipsis {
	return &Ellipsis{
		Ellipsis: SerializePos(pass, input.Ellipsis),
		Elt:      SerProcessExpr(pass, input.Elt),
		Node:     Node{Type: NodeTypeEllipsis, Ref: pass.refCount},
	}
}

func SerProcessExpr(pass *SerPass, expr ast.Expr) Expr {
	switch e := expr.(type) {
	case *ast.BasicLit:
		return SerializeBasicLit(pass, e)
	default:
		return nil
	}
}

func SerProcessSpec(pass *SerPass, spec ast.Spec) Spec {
	switch s := spec.(type) {
	case *ast.ValueSpec:
		return SerializeValueSpec(pass, s)
	default:
		return nil
	}
}

func SerProcessDecl(pass *SerPass, decl ast.Decl) Decl {
	switch d := decl.(type) {
	case *ast.GenDecl:
		return SerializeGenDecl(pass, d)
	default:
		return nil
	}
}
