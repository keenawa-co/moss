package ason

import (
	"go/ast"
	"go/token"
	"os"
)

// TODO List
//
// *ast.BadDecl
// *ast.FuncDecl
// *ast.Package

type SerializationConf int

const (
	// CACHE_REF flag must be used when you carry out some manual manipulations with the source AST tree.
	// For example, you duplicate nodes, which can create nodes that have the same references to the original object in memory.
	//
	// Use this flag when duplicating nodes containing many fields.
	CACHE_REF SerializationConf = iota

	// FILE_SCOPE enable serialization of `Scope` field in `*ast.File`.
	FILE_SCOPE

	// IDENT_OBJ enable serialization of `Obj` field in `*ast.Ident`.
	IDENT_OBJ
)

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

func SerializeList[I ast.Node, R Ason](pass *SerPass, inputList []I, serFn SerFn[I, R]) []R {
	if inputList == nil {
		return nil
	}

	result := make([]R, len(inputList))
	for i := 0; i < len(inputList); i++ {
		result[i] = serFn(pass, inputList[i])
	}

	return result
}

// ----------------- Scope ----------------- //

func SerializeScope(pass *SerPass, input *ast.Scope) *Scope {
	if input == nil {
		return nil
	}

	if pass.conf[FILE_SCOPE] == nil {
		return nil
	}

	var objects map[string]*Object
	if input.Objects != nil {
		objects = make(map[string]*Object, len(input.Objects))
		for k, v := range input.Objects {
			objects[k] = SerializeObject(pass, v)
		}
	}

	return &Scope{
		Outer:   SerializeScope(pass, input.Outer),
		Objects: objects,
	}
}

func SerializeObject(pass *SerPass, input *ast.Object) *Object {
	if input == nil {
		return nil
	}

	if pass.conf[IDENT_OBJ] == nil {
		return nil
	}

	return &Object{
		Kind: input.Kind.String(),
		Name: input.Name,
	}
}

func SerializePos(pass *SerPass, pos token.Pos) Pos {
	if pos == token.NoPos {
		return new(NoPos)
	}

	position := pass.fset.PositionFor(pos, false)

	return &Position{
		Filename: position.Filename,
		Coordinates: [3]int{
			position.Offset,
			position.Line,
			position.Column,
		},
	}
}

// ----------------- Expressions ----------------- //

// TODO: add tests
func SerializeComment(pass *SerPass, input *ast.Comment) *Comment {
	if input == nil {
		return nil
	}

	return &Comment{
		Slash: SerializePos(pass, input.Slash),
		Text:  input.Text,
		Node:  Node{Type: NodeTypeComment, Ref: pass.refCount},
	}
}

// TODO: add tests
func SerializeCommentGroup(pass *SerPass, input *ast.CommentGroup) *CommentGroup {
	if input == nil {
		return nil
	}

	return &CommentGroup{
		List: SerializeList[*ast.Comment, *Comment](pass, input.List, SerializeComment),
		Node: Node{Type: NodeTypeCommentGroup, Ref: pass.refCount},
	}
}

func SerializeIdent(pass *SerPass, input *ast.Ident) *Ident {
	if input == nil {
		return nil
	}

	return &Ident{
		NamePos: SerializePos(pass, input.NamePos),
		Name:    input.String(),
		Obj:     SerializeObject(pass, input.Obj),
		Node:    Node{Type: NodeTypeIdent, Ref: pass.refCount},
	}
}

//
// Everything before this line already have some tests
// <---- TESTED CURSOR
//

func SerializeBasicLit(pass *SerPass, input *ast.BasicLit) *BasicLit {
	if input == nil {
		return nil
	}

	return &BasicLit{
		ValuePos: SerializePos(pass, input.ValuePos),
		Kind:     input.Kind.String(),
		Value:    input.Value,
		Node:     Node{Type: NodeTypeBasicLit, Ref: pass.refCount},
	}
}

func SerializeFuncLit(pass *SerPass, input *ast.FuncLit) *FuncLit {
	if input == nil {
		return nil
	}

	return &FuncLit{
		Type: SerializeFuncType(pass, input.Type),
		Body: SerializeBlockStmt(pass, input.Body),
		Node: Node{Type: NodeTypeFuncLit, Ref: pass.refCount},
	}
}

func SerializeCompositeLit(pass *SerPass, input *ast.CompositeLit) *CompositeLit {
	if input == nil {
		return nil
	}

	return &CompositeLit{
		Type:   SerializeExpr(pass, input.Type),
		Lbrace: SerializePos(pass, input.Lbrace),
		Elts:   SerializeList[ast.Expr, Expr](pass, input.Elts, SerializeExpr),
		Node:   Node{Type: NodeTypeCompositeLit, Ref: pass.refCount},
	}
}

func SerializeField(pass *SerPass, input *ast.Field) *Field {
	if input == nil {
		return nil
	}

	return &Field{
		Names:   SerializeList[*ast.Ident, *Ident](pass, input.Names, SerializeIdent),
		Type:    SerializeExpr(pass, input.Type),
		Tag:     SerializeBasicLit(pass, input.Tag),
		Comment: SerializeCommentGroup(pass, input.Comment),
		Node:    Node{Type: NodeTypeField, Ref: pass.refCount},
	}
}

func SerializeFieldList(pass *SerPass, input *ast.FieldList) *FieldList {
	if input == nil {
		return nil
	}

	return &FieldList{
		Opening: SerializePos(pass, input.Opening),
		List:    SerializeList[*ast.Field, *Field](pass, input.List, SerializeField),
		Closing: SerializePos(pass, input.Closing),
		Node:    Node{Type: NodeTypeFieldList, Ref: pass.refCount},
	}
}

func SerializeEllipsis(pass *SerPass, input *ast.Ellipsis) *Ellipsis {
	if input == nil {
		return nil
	}

	return &Ellipsis{
		Ellipsis: SerializePos(pass, input.Ellipsis),
		Elt:      SerializeExpr(pass, input.Elt),
		Node:     Node{Type: NodeTypeEllipsis, Ref: pass.refCount},
	}
}

func SerializeBadExpr(pass *SerPass, input *ast.BadExpr) *BadExpr {
	if input == nil {
		return nil
	}

	return &BadExpr{
		From: SerializePos(pass, input.From),
		To:   SerializePos(pass, input.To),
		Node: Node{Type: NodeTypeBadExpr, Ref: pass.refCount},
	}
}

func SerializeParenExpr(pass *SerPass, input *ast.ParenExpr) *ParenExpr {
	if input == nil {
		return nil
	}

	return &ParenExpr{
		Lparen: SerializePos(pass, input.Lparen),
		X:      SerializeExpr(pass, input.X),
		Rparen: SerializePos(pass, input.Rparen),
		Node:   Node{Type: NodeTypeParenExpr, Ref: pass.refCount},
	}
}

func SerializeSelectorExpr(pass *SerPass, input *ast.SelectorExpr) *SelectorExpr {
	if input == nil {
		return nil
	}

	return &SelectorExpr{
		X:    SerializeExpr(pass, input.X),
		Sel:  SerializeIdent(pass, input.Sel),
		Node: Node{Type: NodeTypeSelectorExpr, Ref: pass.refCount},
	}
}

func SerializeIndexExpr(pass *SerPass, input *ast.IndexExpr) *IndexExpr {
	if input == nil {
		return nil
	}

	return &IndexExpr{
		X:      SerializeExpr(pass, input.X),
		Lbrack: SerializePos(pass, input.Lbrack),
		Index:  SerializeExpr(pass, input.Index),
		Rbrack: SerializePos(pass, input.Rbrack),
		Node:   Node{Type: NodeTypeIndexExpr, Ref: pass.refCount},
	}
}

func SerializeIndexListExpr(pass *SerPass, input *ast.IndexListExpr) *IndexListExpr {
	if input == nil {
		return nil
	}

	return &IndexListExpr{
		X:       SerializeExpr(pass, input.X),
		Lbrack:  SerializePos(pass, input.Lbrack),
		Indices: SerializeList[ast.Expr, Expr](pass, input.Indices, SerializeExpr),
		Rbrack:  SerializePos(pass, input.Rbrack),
		Node:    Node{Type: NodeTypeIndexListExpr, Ref: pass.refCount},
	}
}

func SerializeSliceExpr(pass *SerPass, input *ast.SliceExpr) *SliceExpr {
	if input == nil {
		return nil
	}

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
	if input == nil {
		return nil
	}

	return &TypeAssertExpr{
		X:      SerializeExpr(pass, input.X),
		Lparen: SerializePos(pass, input.Lparen),
		Type:   SerializeExpr(pass, input.Type),
		Rparen: SerializePos(pass, input.Rparen),
		Node:   Node{Type: NodeTypeTypeAssertExpr, Ref: pass.refCount},
	}
}

func SerializeCallExpr(pass *SerPass, input *ast.CallExpr) *CallExpr {
	if input == nil {
		return nil
	}

	return &CallExpr{
		Fun:      SerializeExpr(pass, input.Fun),
		Lparen:   SerializePos(pass, input.Lparen),
		Args:     SerializeList[ast.Expr, Expr](pass, input.Args, SerializeExpr),
		Ellipsis: SerializePos(pass, input.Ellipsis),
		Rparen:   SerializePos(pass, input.Rparen),
		Node:     Node{Type: NodeTypeCallExpr, Ref: pass.refCount},
	}
}

func SerializeStarExpr(pass *SerPass, input *ast.StarExpr) *StarExpr {
	if input == nil {
		return nil
	}

	return &StarExpr{
		Star: SerializePos(pass, input.Star),
		X:    SerializeExpr(pass, input.X),
		Node: Node{Type: NodeTypeStarExpr, Ref: pass.refCount},
	}
}

func SerializeUnaryExpr(pass *SerPass, input *ast.UnaryExpr) *UnaryExpr {
	if input == nil {
		return nil
	}

	return &UnaryExpr{
		OpPos: SerializePos(pass, input.OpPos),
		Op:    input.Op.String(),
		X:     SerializeExpr(pass, input.X),
		Node:  Node{Type: NodeTypeUnaryExpr, Ref: pass.refCount},
	}
}

func SerializeBinaryExpr(pass *SerPass, input *ast.BinaryExpr) *BinaryExpr {
	if input == nil {
		return nil
	}

	return &BinaryExpr{
		X:     SerializeExpr(pass, input.X),
		OpPos: SerializePos(pass, input.OpPos),
		Op:    input.Op.String(),
		Y:     SerializeExpr(pass, input.Y),
		Node:  Node{Type: NodeTypeBinaryExpr, Ref: pass.refCount},
	}
}

func SerializeKeyValueExpr(pass *SerPass, input *ast.KeyValueExpr) *KeyValueExpr {
	if input == nil {
		return nil
	}

	return &KeyValueExpr{
		Key:   SerializeExpr(pass, input.Key),
		Colon: SerializePos(pass, input.Colon),
		Value: SerializeExpr(pass, input.Value),
		Node:  Node{Type: NodeTypeKeyValueExpr, Ref: pass.refCount},
	}
}

// ----------------- Types ----------------- //

func SerializeArrayType(pass *SerPass, input *ast.ArrayType) *ArrayType {
	if input == nil {
		return nil
	}

	return &ArrayType{
		Lbrack: SerializePos(pass, input.Lbrack),
		Len:    SerializeExpr(pass, input.Len),
		Elt:    SerializeExpr(pass, input.Elt),
		Node:   Node{Type: NodeTypeArrayType, Ref: pass.refCount},
	}
}

func SerializeStructType(pass *SerPass, input *ast.StructType) *StructType {
	if input == nil {
		return nil
	}

	return &StructType{
		Struct:     SerializePos(pass, input.Struct),
		Fields:     SerializeFieldList(pass, input.Fields),
		Incomplete: input.Incomplete,
		Node:       Node{Type: NodeTypeStructType, Ref: pass.refCount},
	}
}

func SerializeFuncType(pass *SerPass, input *ast.FuncType) *FuncType {
	if input == nil {
		return nil
	}

	return &FuncType{
		Func:       SerializePos(pass, input.Func),
		TypeParams: SerializeFieldList(pass, input.TypeParams),
		Params:     SerializeFieldList(pass, input.Params),
		Results:    SerializeFieldList(pass, input.Results),
		Node:       Node{Type: NodeTypeFuncType, Ref: pass.refCount},
	}
}

func SerializeInterfaceType(pass *SerPass, input *ast.InterfaceType) *InterfaceType {
	if input == nil {
		return nil
	}

	return &InterfaceType{
		Interface:  SerializePos(pass, input.Interface),
		Methods:    SerializeFieldList(pass, input.Methods),
		Incomplete: input.Incomplete,
		Node:       Node{Type: NodeTypeInterfaceType, Ref: pass.refCount},
	}
}

func SerializeMapType(pass *SerPass, input *ast.MapType) *MapType {
	if input == nil {
		return nil
	}

	return &MapType{
		Map:   SerializePos(pass, input.Map),
		Key:   SerializeExpr(pass, input.Key),
		Value: SerializeExpr(pass, input.Value),
		Node:  Node{Type: NodeTypeMapType, Ref: pass.refCount},
	}
}

func SerializeChanType(pass *SerPass, input *ast.ChanType) *ChanType {
	if input == nil {
		return nil
	}

	return &ChanType{
		Begin: SerializePos(pass, input.Begin),
		Arrow: SerializePos(pass, input.Arrow),
		Dir:   int(input.Dir),
		Value: SerializeExpr(pass, input.Value),
		Node:  Node{Type: NodeTypeStarExpr, Ref: pass.refCount},
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
	case *ast.FuncLit:
		return SerializeFuncLit(pass, e)
	case *ast.CompositeLit:
		return SerializeCompositeLit(pass, e)
	case *ast.BadExpr:
		return SerializeBadExpr(pass, e)
	case *ast.SelectorExpr:
		return SerializeSelectorExpr(pass, e)
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

	case *ast.ArrayType:
		return SerializeArrayType(pass, e)
	case *ast.StructType:
		return SerializeStructType(pass, e)
	case *ast.FuncType:
		return SerializeFuncType(pass, e)
	case *ast.InterfaceType:
		return SerializeInterfaceType(pass, e)
	case *ast.MapType:
		return SerializeMapType(pass, e)
	case *ast.ChanType:
		return SerializeChanType(pass, e)
	default:
		return nil
	}
}

// ----------------- Statements ----------------- //

func SerializeBadStmt(pass *SerPass, input *ast.BadStmt) *BadStmt {
	if input == nil {
		return nil
	}

	return &BadStmt{
		From: SerializePos(pass, input.From),
		To:   SerializePos(pass, input.To),
		Node: Node{Type: NodeTypeBadStmt, Ref: pass.refCount},
	}
}

func SerializeDeclStmt(pass *SerPass, input *ast.DeclStmt) *DeclStmt {
	if input == nil {
		return nil
	}

	return &DeclStmt{
		Decl: SerializeDecl(pass, input.Decl),
		Node: Node{Type: NodeTypeDeclStmt, Ref: pass.refCount},
	}
}

func SerializeEmptyStmt(pass *SerPass, input *ast.EmptyStmt) *EmptyStmt {
	if input == nil {
		return nil
	}

	return &EmptyStmt{
		Semicolon: SerializePos(pass, input.Semicolon),
		Implicit:  input.Implicit,
		Node:      Node{Type: NodeTypeEmptyStmt, Ref: pass.refCount},
	}
}

func SerializeLabeledStmt(pass *SerPass, input *ast.LabeledStmt) *LabeledStmt {
	if input == nil {
		return nil
	}

	return &LabeledStmt{
		Label: SerializeIdent(pass, input.Label),
		Colon: SerializePos(pass, input.Colon),
		Stmt:  SerializeStmt(pass, input.Stmt),
		Node:  Node{Type: NodeTypeLabeledStmt, Ref: pass.refCount},
	}
}

func SerializeExprStmt(pass *SerPass, input *ast.ExprStmt) *ExprStmt {
	if input == nil {
		return nil
	}

	return &ExprStmt{
		X:    SerializeExpr(pass, input.X),
		Node: Node{Type: NodeTypeExprStmt, Ref: pass.refCount},
	}
}

func SerializeSendStmt(pass *SerPass, input *ast.SendStmt) *SendStmt {
	if input == nil {
		return nil
	}

	return &SendStmt{
		Chan:  SerializeExpr(pass, input.Chan),
		Arrow: SerializePos(pass, input.Arrow),
		Value: SerializeExpr(pass, input.Value),
		Node:  Node{Type: NodeTypeSendStmt, Ref: pass.refCount},
	}
}

func SerializeIncDecStmt(pass *SerPass, input *ast.IncDecStmt) *IncDecStmt {
	if input == nil {
		return nil
	}

	return &IncDecStmt{
		X:      SerializeExpr(pass, input.X),
		TokPos: SerializePos(pass, input.TokPos),
		Tok:    input.Tok.String(),
		Node:   Node{Type: NodeTypeIncDecStmt, Ref: pass.refCount},
	}
}

func SerializeAssignStmt(pass *SerPass, input *ast.AssignStmt) *AssignStmt {
	if input == nil {
		return nil
	}

	return &AssignStmt{
		Lhs:    SerializeList[ast.Expr, Expr](pass, input.Lhs, SerializeExpr),
		TokPos: SerializePos(pass, input.TokPos),
		Tok:    input.Tok.String(),
		Rhs:    SerializeList[ast.Expr, Expr](pass, input.Rhs, SerializeExpr),
		Node:   Node{Type: NodeTypeAssignStmt, Ref: pass.refCount},
	}
}

func SerializeGoStmt(pass *SerPass, input *ast.GoStmt) *GoStmt {
	if input == nil {
		return nil
	}

	return &GoStmt{
		Go:   SerializePos(pass, input.Go),
		Call: SerializeCallExpr(pass, input.Call),
		Node: Node{Type: NodeTypeGoStmt, Ref: pass.refCount},
	}
}

func SerializeDeferStmt(pass *SerPass, input *ast.DeferStmt) *DeferStmt {
	if input == nil {
		return nil
	}

	return &DeferStmt{
		Defer: SerializePos(pass, input.Defer),
		Call:  SerializeCallExpr(pass, input.Call),
		Node:  Node{Type: NodeTypeDeferStmt, Ref: pass.refCount},
	}
}

func SerializeReturnStmt(pass *SerPass, input *ast.ReturnStmt) *ReturnStmt {
	if input == nil {
		return nil
	}

	return &ReturnStmt{
		Return:  SerializePos(pass, input.Return),
		Results: SerializeList[ast.Expr, Expr](pass, input.Results, SerializeExpr),
		Node:    Node{Type: NodeTypeReturnStmt, Ref: pass.refCount},
	}
}

func SerializeBranchStmt(pass *SerPass, input *ast.BranchStmt) *BranchStmt {
	if input == nil {
		return nil
	}

	return &BranchStmt{
		TokPos: SerializePos(pass, input.TokPos),
		Tok:    input.Tok.String(),
		Label:  SerializeIdent(pass, input.Label),
		Node:   Node{Type: NodeTypeBranchStmt, Ref: pass.refCount},
	}
}

func SerializeBlockStmt(pass *SerPass, input *ast.BlockStmt) *BlockStmt {
	if input == nil {
		return nil
	}

	return &BlockStmt{
		Lbrace: SerializePos(pass, input.Lbrace),
		List:   SerializeList[ast.Stmt, Stmt](pass, input.List, SerializeStmt),
		Rbrace: SerializePos(pass, input.Rbrace),
		Node:   Node{Type: NodeTypeBlockStmt, Ref: pass.refCount},
	}
}
func SerializeIfStmt(pass *SerPass, input *ast.IfStmt) *IfStmt {
	if input == nil {
		return nil
	}

	return &IfStmt{
		If:   SerializePos(pass, input.If),
		Init: SerializeStmt(pass, input.Init),
		Cond: SerializeExpr(pass, input.Cond),
		Body: SerializeBlockStmt(pass, input.Body),
		Else: SerializeStmt(pass, input.Else),
		Node: Node{Type: NodeTypeIfStmt, Ref: pass.refCount},
	}
}

func SerializeCaseClause(pass *SerPass, input *ast.CaseClause) *CaseClause {
	if input == nil {
		return nil
	}

	return &CaseClause{
		Case:  SerializePos(pass, input.Case),
		List:  SerializeList[ast.Expr, Expr](pass, input.List, SerializeExpr),
		Colon: SerializePos(pass, input.Colon),
		Body:  SerializeList[ast.Stmt, Stmt](pass, input.Body, SerializeStmt),
		Node:  Node{Type: NodeTypeCaseClause, Ref: pass.refCount},
	}
}

func SerializeSwitchStmt(pass *SerPass, input *ast.SwitchStmt) *SwitchStmt {
	if input == nil {
		return nil
	}

	return &SwitchStmt{
		Switch: SerializePos(pass, input.Switch),
		Init:   SerializeStmt(pass, input.Init),
		Tag:    SerializeExpr(pass, input.Tag),
		Body:   SerializeBlockStmt(pass, input.Body),
		Node:   Node{Type: NodeTypeSwitchStmt, Ref: pass.refCount},
	}
}

func SerializeTypeSwitchStmt(pass *SerPass, input *ast.TypeSwitchStmt) *TypeSwitchStmt {
	if input == nil {
		return nil
	}

	return &TypeSwitchStmt{
		Switch: SerializePos(pass, input.Switch),
		Init:   SerializeStmt(pass, input.Init),
		Assign: SerializeStmt(pass, input.Assign),
		Body:   SerializeBlockStmt(pass, input.Body),
		Node:   Node{Type: NodeTypeExprStmt, Ref: pass.refCount},
	}
}

func SerializeCommClause(pass *SerPass, input *ast.CommClause) *CommClause {
	if input == nil {
		return nil
	}

	return &CommClause{
		Case:  SerializePos(pass, input.Case),
		Comm:  SerializeStmt(pass, input.Comm),
		Colon: SerializePos(pass, input.Colon),
		Body:  SerializeList[ast.Stmt, Stmt](pass, input.Body, SerializeStmt),
		Node:  Node{Type: NodeTypeCommClause, Ref: pass.refCount},
	}
}

func SerializeSelectStmt(pass *SerPass, input *ast.SelectStmt) *SelectStmt {
	if input == nil {
		return nil
	}

	return &SelectStmt{
		Select: SerializePos(pass, input.Select),
		Body:   SerializeBlockStmt(pass, input.Body),
		Node:   Node{Type: NodeTypeSelectStmt, Ref: pass.refCount},
	}
}

func SerializeForStmt(pass *SerPass, input *ast.ForStmt) *ForStmt {
	if input == nil {
		return nil
	}

	return &ForStmt{
		For:  SerializePos(pass, input.For),
		Init: SerializeStmt(pass, input.Init),
		Cond: SerializeExpr(pass, input.Cond),
		Post: SerializeStmt(pass, input.Post),
		Body: SerializeBlockStmt(pass, input.Body),
		Node: Node{Type: NodeTypeForStmt, Ref: pass.refCount},
	}
}

func SerializeRangeStmt(pass *SerPass, input *ast.RangeStmt) *RangeStmt {
	if input == nil {
		return nil
	}

	return &RangeStmt{
		For:    SerializePos(pass, input.For),
		Key:    SerializeExpr(pass, input.Key),
		Value:  SerializeExpr(pass, input.Value),
		TokPos: SerializePos(pass, input.TokPos),
		Tok:    input.Tok.String(),
		Range:  SerializePos(pass, input.Range),
		X:      SerializeExpr(pass, input.X),
		Body:   SerializeBlockStmt(pass, input.Body),
		Node:   Node{Type: NodeTypeExprStmt, Ref: pass.refCount},
	}
}

func SerializeStmt(pass *SerPass, stmt ast.Stmt) Stmt {
	switch s := stmt.(type) {
	case *ast.BadStmt:
		return SerializeBadStmt(pass, s)
	case *ast.DeclStmt:
		return SerializeDeclStmt(pass, s)
	case *ast.EmptyStmt:
		return SerializeEmptyStmt(pass, s)
	case *ast.LabeledStmt:
		return SerializeLabeledStmt(pass, s)
	case *ast.ExprStmt:
		return SerializeExprStmt(pass, s)
	case *ast.SendStmt:
		return SerializeSendStmt(pass, s)
	case *ast.IncDecStmt:
		return SerializeIncDecStmt(pass, s)
	case *ast.AssignStmt:
		return SerializeAssignStmt(pass, s)
	case *ast.GoStmt:
		return SerializeGoStmt(pass, s)
	case *ast.DeferStmt:
		return SerializeDeferStmt(pass, s)
	case *ast.ReturnStmt:
		return SerializeReturnStmt(pass, s)
	case *ast.BranchStmt:
		return SerializeBranchStmt(pass, s)
	case *ast.BlockStmt:
		return SerializeBlockStmt(pass, s)
	case *ast.IfStmt:
		return SerializeIfStmt(pass, s)
	case *ast.CaseClause:
		return SerializeCaseClause(pass, s)
	default:
		return nil
	}
}

// ----------------- Specifications ----------------- //

func SerializeImportSpec(pass *SerPass, input *ast.ImportSpec) *ImportSpec {
	if input == nil {
		return nil
	}

	return &ImportSpec{
		Doc:     SerializeCommentGroup(pass, input.Doc),
		Name:    SerializeIdent(pass, input.Name),
		Path:    SerializeBasicLit(pass, input.Path),
		Comment: SerializeCommentGroup(pass, input.Comment),
		EndPos:  SerializePos(pass, input.EndPos),
		Node:    Node{Type: NodeTypeImportSpec, Ref: pass.refCount},
	}
}

func SerializeValueSpec(pass *SerPass, input *ast.ValueSpec) *ValueSpec {
	if input == nil {
		return nil
	}

	return &ValueSpec{
		Doc:     SerializeCommentGroup(pass, input.Doc),
		Names:   SerializeList[*ast.Ident, *Ident](pass, input.Names, SerializeIdent),
		Type:    SerializeExpr(pass, input.Type),
		Values:  SerializeList[ast.Expr, Expr](pass, input.Values, SerializeExpr),
		Comment: SerializeCommentGroup(pass, input.Comment),
		Node:    Node{Type: NodeTypeValueSpec, Ref: pass.refCount},
	}
}

func SerializeTypeSpec(pass *SerPass, input *ast.TypeSpec) *TypeSpec {
	if input == nil {
		return nil
	}

	return &TypeSpec{
		Doc:        SerializeCommentGroup(pass, input.Doc),
		Name:       SerializeIdent(pass, input.Name),
		TypeParams: SerializeFieldList(pass, input.TypeParams),
		Assign:     SerializePos(pass, input.Assign),
		Type:       SerializeExpr(pass, input.Type),
		Comment:    SerializeCommentGroup(pass, input.Comment),
		Node:       Node{Type: NodeTypeTypeSpec, Ref: pass.refCount},
	}
}

func SerializeSpec(pass *SerPass, spec ast.Spec) Spec {
	switch s := spec.(type) {
	case *ast.ImportSpec:
		return SerializeImportSpec(pass, s)
	case *ast.ValueSpec:
		return SerializeValueSpec(pass, s)
	case *ast.TypeSpec:
		return SerializeTypeSpec(pass, s)
	default:
		return nil
	}
}

// ----------------- Declarations ----------------- //

func SerializeBadDecl(pass *SerPass, input *ast.BadDecl) *BadDecl {
	if input == nil {
		return nil
	}

	return &BadDecl{
		From: SerializePos(pass, input.From),
		To:   SerializePos(pass, input.To),
		Node: Node{Type: NodeTypeBadDecl, Ref: pass.refCount},
	}
}

func SerializeGenDecl(pass *SerPass, input *ast.GenDecl) *GenDecl {
	if input == nil {
		return nil
	}

	if pass.conf[CACHE_REF] != nil {
		return WithRefLookup[*ast.GenDecl, *GenDecl](pass, input, serializeGenDecl)
	}

	return serializeGenDecl(pass, input)
}

func serializeGenDecl(pass *SerPass, input *ast.GenDecl) *GenDecl {
	return &GenDecl{
		Doc:      SerializeCommentGroup(pass, input.Doc),
		TokenPos: SerializePos(pass, input.TokPos),
		Lparen:   SerializePos(pass, input.Lparen),
		Rparen:   SerializePos(pass, input.Rparen),
		Tok:      input.Tok.String(),
		Specs:    SerializeList[ast.Spec, Spec](pass, input.Specs, SerializeSpec),
		Node:     Node{Type: NodeTypeGenDecl, Ref: pass.refCount},
	}
}

func serializeFuncDecl(pass *SerPass, input *ast.FuncDecl) *FuncDecl {
	return &FuncDecl{
		Doc:  SerializeCommentGroup(pass, input.Doc),
		Recv: SerializeFieldList(pass, input.Recv),
		Name: SerializeIdent(pass, input.Name),
		Type: SerializeFuncType(pass, input.Type),
		Body: SerializeBlockStmt(pass, input.Body),
		Node: Node{Type: NodeTypeFuncDecl, Ref: pass.refCount},
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

// ----------------- Files and Packages ----------------- //

func SerializeFile(pass *SerPass, input *ast.File) *File {
	return &File{
		Doc:        SerializeCommentGroup(pass, input.Doc),
		Name:       SerializeIdent(pass, input.Name),
		Decls:      SerializeList[ast.Decl, Decl](pass, input.Decls, SerializeDecl),
		Size:       calcFileSize(pass, input),
		FileStart:  SerializePos(pass, input.FileStart),
		FileEnd:    SerializePos(pass, input.FileEnd),
		Scope:      SerializeScope(pass, input.Scope),
		Imports:    SerializeList[*ast.ImportSpec, *ImportSpec](pass, input.Imports, SerializeImportSpec),
		Unresolved: SerializeList[*ast.Ident, *Ident](pass, input.Unresolved, SerializeIdent),
		Package:    SerializePos(pass, input.Package),
		Comments:   SerializeList[*ast.CommentGroup, *CommentGroup](pass, input.Comments, SerializeCommentGroup),
		GoVersion:  input.GoVersion,
		Node:       Node{Type: NodeTypeFile, Ref: pass.refCount},
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
