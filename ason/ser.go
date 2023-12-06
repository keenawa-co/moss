package ason

import (
	"go/ast"
	"go/token"
	"os"
)

// TODO List
//
// *ast.FuncLit
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
)

type SerConfig struct {
	// RefCounterEnable flag must be used when you carry out some manual
	// manipulations with the source AST tree. For example, you duplicate nodes,
	// which can create nodes that have the same references to the original object in memory.
	//
	// Use this flag when duplicating nodes containing many fields.
	RefCounterEnable bool
}

type SerPass struct {
	fset     *token.FileSet
	readFile func(string) ([]byte, error)
	refCache map[ast.Node]*weakRef
	refCount uint
	conf     map[SerializationConf]interface{}
}

// SerPassOptFn is a functional option type that allows us to configure the SerPass.
type SerPassOptFn func(*SerPass)

func NewSerPass(fset *token.FileSet, options ...SerPassOptFn) *SerPass {
	pass := &SerPass{
		fset:     fset,
		readFile: os.ReadFile,
		conf:     make(map[SerializationConf]interface{}),
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

func WithSerializationConf(options ...SerializationConf) SerPassOptFn {
	return func(pass *SerPass) {
		for i := 0; i < len(options); i++ {
			opt := options[i]
			pass.conf[opt] = struct{}{}

			if opt == CACHE_REF {
				pass.refCache = make(map[ast.Node]*weakRef)
			}
		}
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
		Decls:   SerProcessList[ast.Decl, Decl](pass, input.Decls, SerializeDecl),
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
	return serializePosition(pass, &position)
}

func serializePosition(pass *SerPass, pos *token.Position) *Position {
	return &Position{
		Filename: pos.Filename,
		Coordinates: [3]int{
			pos.Offset,
			pos.Line,
			pos.Column,
		},
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
	return &Ident{
		NamePos: SerializePos(pass, input.NamePos),
		Name:    input.String(),
		Node:    Node{Type: NodeTypeIdent, Ref: pass.refCount},
	}
}

//
// Everything before this line already have some tests
// <---- TESTED CURSOR
//

func SerializeBasicLit(pass *SerPass, input *ast.BasicLit) *BasicLit {
	return &BasicLit{
		ValuePos: SerializePos(pass, input.ValuePos),
		Kind:     input.Kind.String(),
		Value:    input.Value,
		Node:     Node{Type: NodeTypeBasicLit, Ref: pass.refCount},
	}
}

func SerializeCompositeLit(pass *SerPass, input *ast.CompositeLit) *CompositeLit {
	return &CompositeLit{
		Type:   SerializeExpr(pass, input.Type),
		Lbrace: SerializePos(pass, input.Lbrace),
		Elts:   SerProcessList[ast.Expr, Expr](pass, input.Elts, SerializeExpr),
		Node:   Node{Type: NodeTypeCompositeLit, Ref: pass.refCount},
	}
}

func SerializeValueSpec(pass *SerPass, input *ast.ValueSpec) *ValueSpec {
	return &ValueSpec{
		Loc: &Loc{
			Start: SerializePos(pass, input.Pos()),
			End:   SerializePos(pass, input.End()),
		},
		Names:  SerProcessList[*ast.Ident, *Ident](pass, input.Names, SerializeIdent),
		Values: SerProcessList[ast.Expr, Expr](pass, input.Values, SerializeExpr),
		Node:   Node{Type: NodeTypeValueSpec, Ref: pass.refCount},
	}
}

func SerializeGenDecl(pass *SerPass, input *ast.GenDecl) *GenDecl {
	if pass.conf[CACHE_REF] != nil {
		return WithRefLookup[*ast.GenDecl, *GenDecl](pass, input, serializeGenDecl)
	}

	return serializeGenDecl(pass, input)
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
		Specs:    SerProcessList[ast.Spec, Spec](pass, input.Specs, SerializeSpec),
		Node:     Node{Type: NodeTypeGenDecl, Ref: pass.refCount},
	}
}

func SerializeField(pass *SerPass, input *ast.Field) *Field {
	return &Field{
		Names:   SerProcessList[*ast.Ident, *Ident](pass, input.Names, SerializeIdent),
		Type:    SerializeExpr(pass, input.Type),
		Tag:     SerializeBasicLit(pass, input.Tag),
		Comment: SerializeCommentGroup(pass, input.Comment),
		Node:    Node{Type: NodeTypeField, Ref: pass.refCount},
	}
}

func SerializeFieldList(pass *SerPass, input *ast.FieldList) *FieldList {
	return &FieldList{
		Opening: SerializePos(pass, input.Opening),
		List:    SerProcessList[*ast.Field, *Field](pass, input.List, SerializeField),
		Closing: SerializePos(pass, input.Closing),
		Node:    Node{Type: NodeTypeFieldList, Ref: pass.refCount},
	}
}

func SerializeEllipsis(pass *SerPass, input *ast.Ellipsis) *Ellipsis {
	return &Ellipsis{
		Ellipsis: SerializePos(pass, input.Ellipsis),
		Elt:      SerializeExpr(pass, input.Elt),
		Node:     Node{Type: NodeTypeEllipsis, Ref: pass.refCount},
	}
}

func SerializeBadExpr(pass *SerPass, input *ast.BadExpr) *BadExpr {
	return &BadExpr{
		Loc: &Loc{
			Start: SerializePos(pass, input.From),
			End:   SerializePos(pass, input.To),
		},
		Node: Node{Type: NodeTypeBadExpr, Ref: pass.refCount},
	}
}

func SerializeParenExpr(pass *SerPass, input *ast.ParenExpr) *ParenExpr {
	return &ParenExpr{
		Lparen: SerializePos(pass, input.Lparen),
		X:      SerializeExpr(pass, input.X),
		Rparen: SerializePos(pass, input.Rparen),
		Node:   Node{Type: NodeTypeParenExpr, Ref: pass.refCount},
	}
}

func SerializeSelectorExpr(pass *SerPass, input *ast.SelectorExpr) *SelectorExpr {
	return &SelectorExpr{
		X:    SerializeExpr(pass, input.X),
		Sel:  SerializeIdent(pass, input.Sel),
		Node: Node{Type: NodeTypeSelectorExpr, Ref: pass.refCount},
	}
}

func SerializeIndexExpr(pass *SerPass, input *ast.IndexExpr) *IndexExpr {
	return &IndexExpr{
		X:      SerializeExpr(pass, input.X),
		Lbrack: SerializePos(pass, input.Lbrack),
		Index:  SerializeExpr(pass, input.Index),
		Rbrack: SerializePos(pass, input.Rbrack),
		Node:   Node{Type: NodeTypeIndexExpr, Ref: pass.refCount},
	}
}

func SerializeIndexListExpr(pass *SerPass, input *ast.IndexListExpr) *IndexListExpr {
	return &IndexListExpr{
		X:       SerializeExpr(pass, input.X),
		Lbrack:  SerializePos(pass, input.Lbrack),
		Indices: SerProcessList[ast.Expr, Expr](pass, input.Indices, SerializeExpr),
		Rbrack:  SerializePos(pass, input.Rbrack),
		Node:    Node{Type: NodeTypeIndexListExpr, Ref: pass.refCount},
	}
}

func SerializeSliceExpr(pass *SerPass, input *ast.SliceExpr) *SliceExpr {
	return &SliceExpr{
		X:      SerializeExpr(pass, input.X),
		Lbrack: SerializePos(pass, input.Lbrack),
		Low:    SerializeExpr(pass, input.Low),
		High:   SerializeExpr(pass, input.High),
		Max:    SerializeExpr(pass, input.Max),
		Slice3: input.Slice3,
		Rbrack: SerializePos(pass, input.Rbrack),
		Node:   Node{Type: NodeTypeSliceExpr, Ref: pass.refCount},
	}
}

func SerializeTypeAssertExpr(pass *SerPass, input *ast.TypeAssertExpr) *TypeAssertExpr {
	return &TypeAssertExpr{
		X:      SerializeExpr(pass, input.X),
		Lparen: SerializePos(pass, input.Lparen),
		Type:   SerializeExpr(pass, input.Type),
		Rparen: SerializePos(pass, input.Rparen),
		Node:   Node{Type: NodeTypeTypeAssertExpr, Ref: pass.refCount},
	}
}

func SerializeCallExpr(pass *SerPass, input *ast.CallExpr) *CallExpr {
	return &CallExpr{
		Fun:      SerializeExpr(pass, input.Fun),
		Lparen:   SerializePos(pass, input.Lparen),
		Args:     SerProcessList[ast.Expr, Expr](pass, input.Args, SerializeExpr),
		Ellipsis: SerializePos(pass, input.Ellipsis),
		Rparen:   SerializePos(pass, input.Rparen),
		Node:     Node{Type: NodeTypeCallExpr, Ref: pass.refCount},
	}
}

func SerializeStarExpr(pass *SerPass, input *ast.StarExpr) *StarExpr {
	return &StarExpr{
		Star: SerializePos(pass, input.Star),
		X:    SerializeExpr(pass, input.X),
		Node: Node{Type: NodeTypeStarExpr, Ref: pass.refCount},
	}
}

func SerializeUnaryExpr(pass *SerPass, input *ast.UnaryExpr) *UnaryExpr {
	return &UnaryExpr{
		OpPos: SerializePos(pass, input.OpPos),
		Op:    input.Op.String(),
		X:     SerializeExpr(pass, input.X),
		Node:  Node{Type: NodeTypeUnaryExpr, Ref: pass.refCount},
	}
}

func SerializeBinaryExpr(pass *SerPass, input *ast.BinaryExpr) *BinaryExpr {
	return &BinaryExpr{
		X:     SerializeExpr(pass, input.X),
		OpPos: SerializePos(pass, input.OpPos),
		Op:    input.Op.String(),
		Y:     SerializeExpr(pass, input.Y),
		Node:  Node{Type: NodeTypeBinaryExpr, Ref: pass.refCount},
	}
}

func SerializeKeyValueExpr(pass *SerPass, input *ast.KeyValueExpr) *KeyValueExpr {
	return &KeyValueExpr{
		Key:   SerializeExpr(pass, input.Key),
		Colon: SerializePos(pass, input.Colon),
		Value: SerializeExpr(pass, input.Value),
		Node:  Node{Type: NodeTypeKeyValueExpr, Ref: pass.refCount},
	}
}

func SerializeExpr(pass *SerPass, expr ast.Expr) Expr {
	switch e := expr.(type) {
	case *ast.Ident:
		return SerializeIdent(pass, e)
	case *ast.Ellipsis:
		return SerializeEllipsis(pass, e)
	case *ast.BasicLit:
		return SerializeBasicLit(pass, e)
	case *ast.CompositeLit:
		return SerializeCompositeLit(pass, e)
	case *ast.BadExpr:
		return SerializeBadExpr(pass, e)
	case *ast.ParenExpr:
		return SerializeParenExpr(pass, e)
	case *ast.IndexExpr:
		return SerializeIndexExpr(pass, e)
	case *ast.IndexListExpr:
		return SerializeIndexListExpr(pass, e)
	case *ast.UnaryExpr:
		return SerializeUnaryExpr(pass, e)
	case *ast.SliceExpr:
		return SerializeSliceExpr(pass, e)
	case *ast.TypeAssertExpr:
		return SerializeTypeAssertExpr(pass, e)
	case *ast.CallExpr:
		return SerializeCallExpr(pass, e)
	case *ast.StarExpr:
		return SerializeStarExpr(pass, e)
	case *ast.BinaryExpr:
		return SerializeBinaryExpr(pass, e)
	case *ast.KeyValueExpr:
		return SerializeKeyValueExpr(pass, e)
	default:
		return nil
	}
}

func SerializeSpec(pass *SerPass, spec ast.Spec) Spec {
	switch s := spec.(type) {
	case *ast.ValueSpec:
		return SerializeValueSpec(pass, s)
	default:
		return nil
	}
}

func SerializeDecl(pass *SerPass, decl ast.Decl) Decl {
	switch d := decl.(type) {
	case *ast.GenDecl:
		return SerializeGenDecl(pass, d)
	default:
		return nil
	}
}
