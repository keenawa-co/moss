package ason

import (
	"go/ast"
	"go/token"
	"os"
	"unsafe"
)

// TODO List
//
// *ast.Package

type SerializationConf int

const (
	// CACHE_REF flag must be used when you carry out some manual manipulations
	// with the source AST tree. For example, you duplicate nodes, which can create
	// nodes that have the same references to the original object in memory.
	// In order to reduce the number of checks that reduce performance,
	// only large nodes can be cached, such as specifications, types and declarations.
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

// serPassOptFn is a functional option type that allows us to configure the SerPass.
type serPassOptFn func(*SerPass)

func NewSerPass(fset *token.FileSet, options ...serPassOptFn) *SerPass {
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

func WithReadFileFn(fn func(string) ([]byte, error)) serPassOptFn {
	return func(pass *SerPass) {
		pass.readFile = fn
	}
}

func WithSerializationConf(options ...SerializationConf) serPassOptFn {
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

func SerializeOption[I ast.Node, R Ason](pass *SerPass, input I, serFn SerFn[I, R]) (empty R) {
	if *(*interface{})(unsafe.Pointer(&input)) != nil {
		return serFn(pass, input)
	}

	return empty
}

func SerializeList[I ast.Node, R Ason](pass *SerPass, inputList []I, serFn SerFn[I, R]) (result []R) {
	result = make([]R, len(inputList))
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

// ----------------- Comments ----------------- //

// TODO: add tests
func SerializeComment(pass *SerPass, input *ast.Comment) *Comment {
	return &Comment{
		Slash: SerializePos(pass, input.Slash),
		Text:  input.Text,
		Node:  Node{Type: NodeTypeComment, Ref: pass.refCount},
	}
}

// TODO: add tests
func SerializeCommentGroup(pass *SerPass, input *ast.CommentGroup) *CommentGroup {
	return &CommentGroup{
		List: SerializeList[*ast.Comment, *Comment](pass, input.List, SerializeComment),
		Node: Node{Type: NodeTypeCommentGroup, Ref: pass.refCount},
	}
}

// ----------------- Expressions ----------------- //

func SerializeIdent(pass *SerPass, input *ast.Ident) *Ident {
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
	return &BasicLit{
		ValuePos: SerializePos(pass, input.ValuePos),
		Kind:     input.Kind.String(),
		Value:    input.Value,
		Node:     Node{Type: NodeTypeBasicLit, Ref: pass.refCount},
	}
}

func SerializeFuncLit(pass *SerPass, input *ast.FuncLit) *FuncLit {
	return &FuncLit{
		Type: SerializeOption(pass, input.Type, SerializeFuncType),
		Body: SerializeOption(pass, input.Body, SerializeBlockStmt),
		Node: Node{Type: NodeTypeFuncLit, Ref: pass.refCount},
	}
}

func SerializeCompositeLit(pass *SerPass, input *ast.CompositeLit) *CompositeLit {
	return &CompositeLit{
		Type:   SerializeOption(pass, input.Type, SerializeExpr),
		Lbrace: SerializePos(pass, input.Lbrace),
		Elts:   SerializeList(pass, input.Elts, SerializeExpr),
		Node:   Node{Type: NodeTypeCompositeLit, Ref: pass.refCount},
	}
}

func SerializeField(pass *SerPass, input *ast.Field) *Field {
	return &Field{
		Doc:     SerializeOption(pass, input.Doc, SerializeCommentGroup),
		Names:   SerializeList(pass, input.Names, SerializeIdent),
		Type:    SerializeOption(pass, input.Type, SerializeExpr),
		Tag:     SerializeOption(pass, input.Tag, SerializeBasicLit),
		Comment: SerializeOption(pass, input.Comment, SerializeCommentGroup),
		Node:    Node{Type: NodeTypeField, Ref: pass.refCount},
	}
}

func SerializeFieldList(pass *SerPass, input *ast.FieldList) *FieldList {
	return &FieldList{
		Opening: SerializePos(pass, input.Opening),
		List:    SerializeList(pass, input.List, SerializeField),
		Closing: SerializePos(pass, input.Closing),
		Node:    Node{Type: NodeTypeFieldList, Ref: pass.refCount},
	}
}

func SerializeEllipsis(pass *SerPass, input *ast.Ellipsis) *Ellipsis {
	return &Ellipsis{
		Ellipsis: SerializePos(pass, input.Ellipsis),
		Elt:      SerializeOption(pass, input.Elt, SerializeExpr),
		Node:     Node{Type: NodeTypeEllipsis, Ref: pass.refCount},
	}
}

func SerializeBadExpr(pass *SerPass, input *ast.BadExpr) *BadExpr {
	return &BadExpr{
		From: SerializePos(pass, input.From),
		To:   SerializePos(pass, input.To),
		Node: Node{Type: NodeTypeBadExpr, Ref: pass.refCount},
	}
}

func SerializeParenExpr(pass *SerPass, input *ast.ParenExpr) *ParenExpr {
	return &ParenExpr{
		Lparen: SerializePos(pass, input.Lparen),
		X:      SerializeOption(pass, input.X, SerializeExpr),
		Rparen: SerializePos(pass, input.Rparen),
		Node:   Node{Type: NodeTypeParenExpr, Ref: pass.refCount},
	}
}

func SerializeSelectorExpr(pass *SerPass, input *ast.SelectorExpr) *SelectorExpr {
	return &SelectorExpr{
		X:    SerializeOption(pass, input.X, SerializeExpr),
		Sel:  SerializeOption(pass, input.Sel, SerializeIdent),
		Node: Node{Type: NodeTypeSelectorExpr, Ref: pass.refCount},
	}
}

func SerializeIndexExpr(pass *SerPass, input *ast.IndexExpr) *IndexExpr {
	return &IndexExpr{
		X:      SerializeOption(pass, input.X, SerializeExpr),
		Lbrack: SerializePos(pass, input.Lbrack),
		Index:  SerializeOption(pass, input.Index, SerializeExpr),
		Rbrack: SerializePos(pass, input.Rbrack),
		Node:   Node{Type: NodeTypeIndexExpr, Ref: pass.refCount},
	}
}

func SerializeIndexListExpr(pass *SerPass, input *ast.IndexListExpr) *IndexListExpr {
	return &IndexListExpr{
		X:       SerializeOption(pass, input.X, SerializeExpr),
		Lbrack:  SerializePos(pass, input.Lbrack),
		Indices: SerializeList(pass, input.Indices, SerializeExpr),
		Rbrack:  SerializePos(pass, input.Rbrack),
		Node:    Node{Type: NodeTypeIndexListExpr, Ref: pass.refCount},
	}
}

func SerializeSliceExpr(pass *SerPass, input *ast.SliceExpr) *SliceExpr {
	return &SliceExpr{
		X:      SerializeOption(pass, input.X, SerializeExpr),
		Lbrack: SerializePos(pass, input.Lbrack),
		Low:    SerializeOption(pass, input.Low, SerializeExpr),
		High:   SerializeOption(pass, input.High, SerializeExpr),
		Max:    SerializeOption(pass, input.Max, SerializeExpr),
		Slice3: input.Slice3,
		Rbrack: SerializePos(pass, input.Rbrack),
		Node:   Node{Type: NodeTypeSliceExpr, Ref: pass.refCount},
	}
}

func SerializeTypeAssertExpr(pass *SerPass, input *ast.TypeAssertExpr) *TypeAssertExpr {
	return &TypeAssertExpr{
		X:      SerializeOption(pass, input.X, SerializeExpr),
		Lparen: SerializePos(pass, input.Lparen),
		Type:   SerializeOption(pass, input.Type, SerializeExpr),
		Rparen: SerializePos(pass, input.Rparen),
		Node:   Node{Type: NodeTypeTypeAssertExpr, Ref: pass.refCount},
	}
}

func SerializeCallExpr(pass *SerPass, input *ast.CallExpr) *CallExpr {
	return &CallExpr{
		Fun:      SerializeOption(pass, input.Fun, SerializeExpr),
		Lparen:   SerializePos(pass, input.Lparen),
		Args:     SerializeList(pass, input.Args, SerializeExpr),
		Ellipsis: SerializePos(pass, input.Ellipsis),
		Rparen:   SerializePos(pass, input.Rparen),
		Node:     Node{Type: NodeTypeCallExpr, Ref: pass.refCount},
	}
}

func SerializeStarExpr(pass *SerPass, input *ast.StarExpr) *StarExpr {
	return &StarExpr{
		Star: SerializePos(pass, input.Star),
		X:    SerializeOption(pass, input.X, SerializeExpr),
		Node: Node{Type: NodeTypeStarExpr, Ref: pass.refCount},
	}
}

func SerializeUnaryExpr(pass *SerPass, input *ast.UnaryExpr) *UnaryExpr {
	return &UnaryExpr{
		OpPos: SerializePos(pass, input.OpPos),
		Op:    input.Op.String(),
		X:     SerializeOption(pass, input.X, SerializeExpr),
		Node:  Node{Type: NodeTypeUnaryExpr, Ref: pass.refCount},
	}
}

func SerializeBinaryExpr(pass *SerPass, input *ast.BinaryExpr) *BinaryExpr {
	return &BinaryExpr{
		X:     SerializeOption(pass, input.X, SerializeExpr),
		OpPos: SerializePos(pass, input.OpPos),
		Op:    input.Op.String(),
		Y:     SerializeOption(pass, input.Y, SerializeExpr),
		Node:  Node{Type: NodeTypeBinaryExpr, Ref: pass.refCount},
	}
}

func SerializeKeyValueExpr(pass *SerPass, input *ast.KeyValueExpr) *KeyValueExpr {
	return &KeyValueExpr{
		Key:   SerializeOption(pass, input.Key, SerializeExpr),
		Colon: SerializePos(pass, input.Colon),
		Value: SerializeOption(pass, input.Value, SerializeExpr),
		Node:  Node{Type: NodeTypeKeyValueExpr, Ref: pass.refCount},
	}
}

// ----------------- Types ----------------- //

func SerializeArrayType(pass *SerPass, input *ast.ArrayType) *ArrayType {
	return &ArrayType{
		Lbrack: SerializePos(pass, input.Lbrack),
		Len:    SerializeOption(pass, input.Len, SerializeExpr),
		Elt:    SerializeOption(pass, input.Elt, SerializeExpr),
		Node:   Node{Type: NodeTypeArrayType, Ref: pass.refCount},
	}
}

func SerializeStructType(pass *SerPass, input *ast.StructType) *StructType {
	return &StructType{
		Struct:     SerializePos(pass, input.Struct),
		Fields:     SerializeOption(pass, input.Fields, SerializeFieldList),
		Incomplete: input.Incomplete,
		Node:       Node{Type: NodeTypeStructType, Ref: pass.refCount},
	}
}

func SerializeFuncType(pass *SerPass, input *ast.FuncType) *FuncType {
	return &FuncType{
		Func:       SerializePos(pass, input.Func),
		TypeParams: SerializeOption(pass, input.TypeParams, SerializeFieldList),
		Params:     SerializeOption(pass, input.Params, SerializeFieldList),
		Results:    SerializeOption(pass, input.Results, SerializeFieldList),
		Node:       Node{Type: NodeTypeFuncType, Ref: pass.refCount},
	}
}

func SerializeInterfaceType(pass *SerPass, input *ast.InterfaceType) *InterfaceType {
	return &InterfaceType{
		Interface:  SerializePos(pass, input.Interface),
		Methods:    SerializeOption(pass, input.Methods, SerializeFieldList),
		Incomplete: input.Incomplete,
		Node:       Node{Type: NodeTypeInterfaceType, Ref: pass.refCount},
	}
}

func SerializeMapType(pass *SerPass, input *ast.MapType) *MapType {
	return &MapType{
		Map:   SerializePos(pass, input.Map),
		Key:   SerializeOption(pass, input.Key, SerializeExpr),
		Value: SerializeOption(pass, input.Value, SerializeExpr),
		Node:  Node{Type: NodeTypeMapType, Ref: pass.refCount},
	}
}

func SerializeChanType(pass *SerPass, input *ast.ChanType) *ChanType {
	return &ChanType{
		Begin: SerializePos(pass, input.Begin),
		Arrow: SerializePos(pass, input.Arrow),
		Dir:   int(input.Dir),
		Value: SerializeOption(pass, input.Value, SerializeExpr),
		Node:  Node{Type: NodeTypeChanType, Ref: pass.refCount},
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
	return &BadStmt{
		From: SerializePos(pass, input.From),
		To:   SerializePos(pass, input.To),
		Node: Node{Type: NodeTypeBadStmt, Ref: pass.refCount},
	}
}

func SerializeDeclStmt(pass *SerPass, input *ast.DeclStmt) *DeclStmt {
	return &DeclStmt{
		Decl: SerializeOption(pass, input.Decl, SerializeDecl),
		Node: Node{Type: NodeTypeDeclStmt, Ref: pass.refCount},
	}
}

func SerializeEmptyStmt(pass *SerPass, input *ast.EmptyStmt) *EmptyStmt {
	return &EmptyStmt{
		Semicolon: SerializePos(pass, input.Semicolon),
		Implicit:  input.Implicit,
		Node:      Node{Type: NodeTypeEmptyStmt, Ref: pass.refCount},
	}
}

func SerializeLabeledStmt(pass *SerPass, input *ast.LabeledStmt) *LabeledStmt {
	return &LabeledStmt{
		Label: SerializeOption(pass, input.Label, SerializeIdent),
		Colon: SerializePos(pass, input.Colon),
		Stmt:  SerializeOption(pass, input.Stmt, SerializeStmt),
		Node:  Node{Type: NodeTypeLabeledStmt, Ref: pass.refCount},
	}
}

func SerializeExprStmt(pass *SerPass, input *ast.ExprStmt) *ExprStmt {
	return &ExprStmt{
		X:    SerializeOption(pass, input.X, SerializeExpr),
		Node: Node{Type: NodeTypeExprStmt, Ref: pass.refCount},
	}
}

func SerializeSendStmt(pass *SerPass, input *ast.SendStmt) *SendStmt {
	return &SendStmt{
		Chan:  SerializeOption(pass, input.Chan, SerializeExpr),
		Arrow: SerializePos(pass, input.Arrow),
		Value: SerializeOption(pass, input.Value, SerializeExpr),
		Node:  Node{Type: NodeTypeSendStmt, Ref: pass.refCount},
	}
}

func SerializeIncDecStmt(pass *SerPass, input *ast.IncDecStmt) *IncDecStmt {
	return &IncDecStmt{
		X:      SerializeOption(pass, input.X, SerializeExpr),
		TokPos: SerializePos(pass, input.TokPos),
		Tok:    input.Tok.String(),
		Node:   Node{Type: NodeTypeIncDecStmt, Ref: pass.refCount},
	}
}

func SerializeAssignStmt(pass *SerPass, input *ast.AssignStmt) *AssignStmt {
	return &AssignStmt{
		Lhs:    SerializeList(pass, input.Lhs, SerializeExpr),
		TokPos: SerializePos(pass, input.TokPos),
		Tok:    input.Tok.String(),
		Rhs:    SerializeList(pass, input.Rhs, SerializeExpr),
		Node:   Node{Type: NodeTypeAssignStmt, Ref: pass.refCount},
	}
}

func SerializeGoStmt(pass *SerPass, input *ast.GoStmt) *GoStmt {
	return &GoStmt{
		Go:   SerializePos(pass, input.Go),
		Call: SerializeOption(pass, input.Call, SerializeCallExpr),
		Node: Node{Type: NodeTypeGoStmt, Ref: pass.refCount},
	}
}

func SerializeDeferStmt(pass *SerPass, input *ast.DeferStmt) *DeferStmt {
	return &DeferStmt{
		Defer: SerializePos(pass, input.Defer),
		Call:  SerializeOption(pass, input.Call, SerializeCallExpr),
		Node:  Node{Type: NodeTypeDeferStmt, Ref: pass.refCount},
	}
}

func SerializeReturnStmt(pass *SerPass, input *ast.ReturnStmt) *ReturnStmt {
	return &ReturnStmt{
		Return:  SerializePos(pass, input.Return),
		Results: SerializeList(pass, input.Results, SerializeExpr),
		Node:    Node{Type: NodeTypeReturnStmt, Ref: pass.refCount},
	}
}

func SerializeBranchStmt(pass *SerPass, input *ast.BranchStmt) *BranchStmt {
	return &BranchStmt{
		TokPos: SerializePos(pass, input.TokPos),
		Tok:    input.Tok.String(),
		Label:  SerializeOption(pass, input.Label, SerializeIdent),
		Node:   Node{Type: NodeTypeBranchStmt, Ref: pass.refCount},
	}
}

func SerializeBlockStmt(pass *SerPass, input *ast.BlockStmt) *BlockStmt {
	return &BlockStmt{
		Lbrace: SerializePos(pass, input.Lbrace),
		List:   SerializeList(pass, input.List, SerializeStmt),
		Rbrace: SerializePos(pass, input.Rbrace),
		Node:   Node{Type: NodeTypeBlockStmt, Ref: pass.refCount},
	}
}

func SerializeIfStmt(pass *SerPass, input *ast.IfStmt) *IfStmt {
	return &IfStmt{
		If:   SerializePos(pass, input.If),
		Init: SerializeOption(pass, input.Init, SerializeStmt),
		Cond: SerializeOption(pass, input.Cond, SerializeExpr),
		Body: SerializeOption(pass, input.Body, SerializeBlockStmt),
		Else: SerializeOption(pass, input.Else, SerializeStmt),
		Node: Node{Type: NodeTypeIfStmt, Ref: pass.refCount},
	}
}

func SerializeCaseClause(pass *SerPass, input *ast.CaseClause) *CaseClause {
	return &CaseClause{
		Case:  SerializePos(pass, input.Case),
		List:  SerializeList(pass, input.List, SerializeExpr),
		Colon: SerializePos(pass, input.Colon),
		Body:  SerializeList(pass, input.Body, SerializeStmt),
		Node:  Node{Type: NodeTypeCaseClause, Ref: pass.refCount},
	}
}

func SerializeSwitchStmt(pass *SerPass, input *ast.SwitchStmt) *SwitchStmt {
	return &SwitchStmt{
		Switch: SerializePos(pass, input.Switch),
		Init:   SerializeOption(pass, input.Init, SerializeStmt),
		Tag:    SerializeOption(pass, input.Tag, SerializeExpr),
		Body:   SerializeOption(pass, input.Body, SerializeBlockStmt),
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
	return &CommClause{
		Case:  SerializePos(pass, input.Case),
		Comm:  SerializeOption(pass, input.Comm, SerializeStmt),
		Colon: SerializePos(pass, input.Colon),
		Body:  SerializeList(pass, input.Body, SerializeStmt),
		Node:  Node{Type: NodeTypeCommClause, Ref: pass.refCount},
	}
}

func SerializeSelectStmt(pass *SerPass, input *ast.SelectStmt) *SelectStmt {
	return &SelectStmt{
		Select: SerializePos(pass, input.Select),
		Body:   SerializeOption(pass, input.Body, SerializeBlockStmt),
		Node:   Node{Type: NodeTypeSelectStmt, Ref: pass.refCount},
	}
}

func SerializeForStmt(pass *SerPass, input *ast.ForStmt) *ForStmt {
	return &ForStmt{
		For:  SerializePos(pass, input.For),
		Init: SerializeOption(pass, input.Init, SerializeStmt),
		Cond: SerializeOption(pass, input.Cond, SerializeExpr),
		Post: SerializeOption(pass, input.Post, SerializeStmt),
		Body: SerializeOption(pass, input.Body, SerializeBlockStmt),
		Node: Node{Type: NodeTypeForStmt, Ref: pass.refCount},
	}
}

func SerializeRangeStmt(pass *SerPass, input *ast.RangeStmt) *RangeStmt {
	return &RangeStmt{
		For:    SerializePos(pass, input.For),
		Key:    SerializeOption(pass, input.Key, SerializeExpr),
		Value:  SerializeOption(pass, input.Value, SerializeExpr),
		TokPos: SerializePos(pass, input.TokPos),
		Tok:    input.Tok.String(),
		Range:  SerializePos(pass, input.Range),
		X:      SerializeOption(pass, input.X, SerializeExpr),
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
	if pass.conf[CACHE_REF] != nil {
		return WithRefLookup(pass, input, serializeImportSpec)
	}

	return serializeImportSpec(pass, input)
}

func serializeImportSpec(pass *SerPass, input *ast.ImportSpec) *ImportSpec {
	return &ImportSpec{
		Doc:     SerializeOption(pass, input.Doc, SerializeCommentGroup),
		Name:    SerializeOption(pass, input.Name, SerializeIdent),
		Path:    SerializeOption(pass, input.Path, SerializeBasicLit),
		Comment: SerializeOption(pass, input.Comment, SerializeCommentGroup),
		EndPos:  SerializePos(pass, input.EndPos),
		Node:    Node{Type: NodeTypeImportSpec, Ref: pass.refCount},
	}
}

func SerializeValueSpec(pass *SerPass, input *ast.ValueSpec) *ValueSpec {
	if pass.conf[CACHE_REF] != nil {
		return WithRefLookup(pass, input, serializeValueSpec)
	}

	return serializeValueSpec(pass, input)
}

func serializeValueSpec(pass *SerPass, input *ast.ValueSpec) *ValueSpec {
	return &ValueSpec{
		Doc:     SerializeOption(pass, input.Doc, SerializeCommentGroup),
		Names:   SerializeList(pass, input.Names, SerializeIdent),
		Type:    SerializeOption(pass, input.Type, SerializeExpr),
		Values:  SerializeList(pass, input.Values, SerializeExpr),
		Comment: SerializeOption(pass, input.Comment, SerializeCommentGroup),
		Node:    Node{Type: NodeTypeValueSpec, Ref: pass.refCount},
	}
}

func SerializeTypeSpec(pass *SerPass, input *ast.TypeSpec) *TypeSpec {
	if pass.conf[CACHE_REF] != nil {
		return WithRefLookup(pass, input, serializeTypeSpec)
	}

	return serializeTypeSpec(pass, input)
}

func serializeTypeSpec(pass *SerPass, input *ast.TypeSpec) *TypeSpec {
	return &TypeSpec{
		Doc:        SerializeOption(pass, input.Doc, SerializeCommentGroup),
		Name:       SerializeOption(pass, input.Name, SerializeIdent),
		TypeParams: SerializeOption(pass, input.TypeParams, SerializeFieldList),
		Assign:     SerializePos(pass, input.Assign),
		Type:       SerializeOption(pass, input.Type, SerializeExpr),
		Comment:    SerializeOption(pass, input.Comment, SerializeCommentGroup),
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
	if pass.conf[CACHE_REF] != nil {
		return WithRefLookup(pass, input, serializeBadDecl)
	}

	return serializeBadDecl(pass, input)
}

func serializeBadDecl(pass *SerPass, input *ast.BadDecl) *BadDecl {
	return &BadDecl{
		From: SerializePos(pass, input.From),
		To:   SerializePos(pass, input.To),
		Node: Node{Type: NodeTypeBadDecl, Ref: pass.refCount},
	}
}

func SerializeGenDecl(pass *SerPass, input *ast.GenDecl) *GenDecl {
	if pass.conf[CACHE_REF] != nil {
		return WithRefLookup(pass, input, serializeGenDecl)
	}

	return serializeGenDecl(pass, input)
}

func serializeGenDecl(pass *SerPass, input *ast.GenDecl) *GenDecl {
	return &GenDecl{
		Doc:      SerializeOption(pass, input.Doc, SerializeCommentGroup),
		TokenPos: SerializePos(pass, input.TokPos),
		Lparen:   SerializePos(pass, input.Lparen),
		Rparen:   SerializePos(pass, input.Rparen),
		Tok:      input.Tok.String(),
		Specs:    SerializeList(pass, input.Specs, SerializeSpec),
		Node:     Node{Type: NodeTypeGenDecl, Ref: pass.refCount},
	}
}

func SerializeFuncDecl(pass *SerPass, input *ast.FuncDecl) *FuncDecl {
	if pass.conf[CACHE_REF] != nil {
		return WithRefLookup(pass, input, serializeFuncDecl)
	}

	return serializeFuncDecl(pass, input)
}

func serializeFuncDecl(pass *SerPass, input *ast.FuncDecl) *FuncDecl {
	return &FuncDecl{
		Doc:  SerializeOption(pass, input.Doc, SerializeCommentGroup),
		Recv: SerializeOption(pass, input.Recv, SerializeFieldList),
		Name: SerializeOption(pass, input.Name, SerializeIdent),
		Type: SerializeOption(pass, input.Type, SerializeFuncType),
		Body: SerializeOption(pass, input.Body, SerializeBlockStmt),
		Node: Node{Type: NodeTypeFuncDecl, Ref: pass.refCount},
	}
}

func SerializeDecl(pass *SerPass, decl ast.Decl) Decl {
	switch d := decl.(type) {
	case *ast.BadDecl:
		return SerializeBadDecl(pass, d)
	case *ast.GenDecl:
		return SerializeGenDecl(pass, d)
	case *ast.FuncDecl:
		return SerializeFuncDecl(pass, d)
	default:
		return nil
	}
}

// ----------------- Files and Packages ----------------- //

func SerializeFile(pass *SerPass, input *ast.File) *File {
	return &File{
		Doc:        SerializeOption(pass, input.Doc, SerializeCommentGroup),
		Name:       SerializeOption(pass, input.Name, SerializeIdent),
		Decls:      SerializeList(pass, input.Decls, SerializeDecl),
		Size:       calcFileSize(pass, input),
		FileStart:  SerializePos(pass, input.FileStart),
		FileEnd:    SerializePos(pass, input.FileEnd),
		Scope:      SerializeScope(pass, input.Scope),
		Imports:    SerializeList(pass, input.Imports, SerializeImportSpec),
		Unresolved: SerializeList(pass, input.Unresolved, SerializeIdent),
		Package:    SerializePos(pass, input.Package),
		Comments:   SerializeList(pass, input.Comments, SerializeCommentGroup),
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
