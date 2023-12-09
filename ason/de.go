package ason

import (
	"fmt"
	"go/ast"
	"go/token"
	"log"
	"unsafe"
)

// TODO List
//
// *ast.Package

type dePass struct {
	fset     *token.FileSet
	refCache map[ast.Node]*weakRef
}

func NewDePass(fset *token.FileSet) *dePass {
	return &dePass{
		fset: fset,
	}
}

type DeFn[I Ason, R ast.Node] func(*dePass, I) R

func DeserializeOption[I Ason, R ast.Node](pass *dePass, input I, deFn DeFn[I, R]) (empty R) {
	if *(*interface{})(unsafe.Pointer(&input)) != nil {
		return deFn(pass, input)
	}

	return empty
}

func DeserializeList[I Ason, R ast.Node](pass *dePass, inputList []I, deFn DeFn[I, R]) []R {
	result := make([]R, len(inputList))
	for i := 0; i < len(inputList); i++ {
		result[i] = deFn(pass, inputList[i])
	}

	return result
}

// ----------------- Scope ----------------- //

func DeserializePos(pass *dePass, input Pos) token.Pos {
	pos, ok := input.(*Position)
	if !ok {
		return token.NoPos
	}

	tokPos := token.Pos(pos.Offset())
	tokFile := pass.fset.File(tokPos)
	if tokFile == nil {
		return token.NoPos
	}

	return tokFile.Pos(pos.Offset())
}

// ----------------- Comments ----------------- //

func DeserializeComment(pass *dePass, input *Comment) *ast.Comment {
	return &ast.Comment{
		Slash: DeserializePos(pass, input.Slash),
		Text:  input.Text,
	}
}

func DeserializeCommentGroup(pass *dePass, input *CommentGroup) *ast.CommentGroup {
	if input == nil {
		return nil
	}

	return &ast.CommentGroup{
		List: DeserializeList[*Comment, *ast.Comment](pass, input.List, DeserializeComment),
	}
}

// ----------------- Expressions ----------------- //

func DeserializeField(pass *dePass, input *Field) *ast.Field {
	if input == nil {
		return nil
	}

	return &ast.Field{
		Doc:     DeserializeCommentGroup(pass, input.Doc),
		Names:   DeserializeList[*Ident, *ast.Ident](pass, input.Names, DeserializeIdent),
		Type:    DeserializeExpr(pass, input.Type),
		Tag:     DeserializeBasicLit(pass, input.Tag),
		Comment: DeserializeCommentGroup(pass, input.Comment),
	}
}

func DeserializeFieldList(pass *dePass, input *FieldList) *ast.FieldList {
	if input == nil {
		return nil
	}

	return &ast.FieldList{
		Opening: DeserializePos(pass, input.Opening),
		List:    DeserializeList[*Field, *ast.Field](pass, input.List, DeserializeField),
		Closing: DeserializePos(pass, input.Closing),
	}
}

func DeserializeIdent(pass *dePass, input *Ident) *ast.Ident {
	if input == nil {
		return nil
	}

	return &ast.Ident{
		Name:    input.Name,
		NamePos: DeserializePos(pass, input.NamePos),
	}
}

func DeserializeBasicLit(pass *dePass, input *BasicLit) *ast.BasicLit {
	if input == nil {
		return nil
	}

	return &ast.BasicLit{
		ValuePos: DeserializePos(pass, input.ValuePos),
		Kind:     tokens[input.Kind],
		Value:    input.Value,
	}
}

func DeserializeFuncLit(pass *dePass, input *FuncLit) *ast.FuncLit {
	if input == nil {
		return nil
	}

	return &ast.FuncLit{
		Type: DeserializeFuncType(pass, input.Type),
		Body: DeserializeBlockStmt(pass, input.Body),
	}
}

func DeserializeCompositeLit(pass *dePass, input *CompositeLit) *ast.CompositeLit {
	if input == nil {
		return nil
	}

	return &ast.CompositeLit{
		Type:       DeserializeExpr(pass, input.Type),
		Lbrace:     DeserializePos(pass, input.Lbrace),
		Elts:       DeserializeList[Expr, ast.Expr](pass, input.Elts, DeserializeExpr),
		Rbrace:     DeserializePos(pass, input.Rbrace),
		Incomplete: input.Incomplete,
	}
}

func DeserializeEllipsis(pass *dePass, input *Ellipsis) *ast.Ellipsis {
	if input == nil {
		return nil
	}

	return &ast.Ellipsis{
		Ellipsis: DeserializePos(pass, input.Ellipsis),
		Elt:      DeserializeExpr(pass, input.Elt),
	}
}

func DeserializeBadExpr(pass *dePass, input *BadExpr) *ast.BadExpr {
	if input == nil {
		return nil
	}

	return &ast.BadExpr{
		From: DeserializePos(pass, input.From),
		To:   DeserializePos(pass, input.To),
	}
}

func DeserializeParenExpr(pass *dePass, input *ParenExpr) *ast.ParenExpr {
	return &ast.ParenExpr{
		Lparen: DeserializePos(pass, input.Lparen),
		X:      DeserializeExpr(pass, input.X),
		Rparen: DeserializePos(pass, input.Rparen),
	}
}

func DeserializeSelectorExpr(pass *dePass, input *SelectorExpr) *ast.SelectorExpr {
	return &ast.SelectorExpr{
		X:   DeserializeExpr(pass, input.X),
		Sel: DeserializeIdent(pass, input.Sel),
	}
}

func DeserializeIndexExpr(pass *dePass, input *IndexExpr) *ast.IndexExpr {
	return &ast.IndexExpr{
		X:      DeserializeExpr(pass, input.X),
		Lbrack: DeserializePos(pass, input.Lbrack),
		Index:  DeserializeExpr(pass, input.Index),
		Rbrack: DeserializePos(pass, input.Rbrack),
	}
}

func DeserializeIndexListExpr(pass *dePass, input *IndexListExpr) *ast.IndexListExpr {
	return &ast.IndexListExpr{
		X:       DeserializeExpr(pass, input.X),
		Lbrack:  DeserializePos(pass, input.Lbrack),
		Indices: DeserializeList[Expr, ast.Expr](pass, input.Indices, DeserializeExpr),
		Rbrack:  DeserializePos(pass, input.Rbrack),
	}
}

func DeserializeSliceExpr(pass *dePass, input *SliceExpr) *ast.SliceExpr {
	return &ast.SliceExpr{
		X:      DeserializeExpr(pass, input.X),
		Lbrack: DeserializePos(pass, input.Lbrack),
		Low:    DeserializeExpr(pass, input.Low),
		High:   DeserializeExpr(pass, input.High),
		Max:    DeserializeExpr(pass, input.Max),
		Slice3: input.Slice3,
		Rbrack: DeserializePos(pass, input.Rbrack),
	}
}

func DeserializeTypeAssertExpr(pass *dePass, input *TypeAssertExpr) *ast.TypeAssertExpr {
	return &ast.TypeAssertExpr{
		X:      DeserializeExpr(pass, input.X),
		Lparen: DeserializePos(pass, input.Lparen),
		Type:   DeserializeExpr(pass, input.Type),
		Rparen: DeserializePos(pass, input.Rparen),
	}
}

func DeserializeCallExpr(pass *dePass, input *CallExpr) *ast.CallExpr {
	return &ast.CallExpr{
		Fun:      DeserializeExpr(pass, input.Fun),
		Lparen:   DeserializePos(pass, input.Lparen),
		Args:     DeserializeList[Expr, ast.Expr](pass, input.Args, DeserializeExpr),
		Ellipsis: DeserializePos(pass, input.Ellipsis),
		Rparen:   DeserializePos(pass, input.Rparen),
	}
}

func DeserializeStarExpr(pass *dePass, input *StarExpr) *ast.StarExpr {
	return &ast.StarExpr{
		Star: DeserializePos(pass, input.Star),
		X:    DeserializeExpr(pass, input.X),
	}
}

func DeserializeUnaryExpr(pass *dePass, input *UnaryExpr) *ast.UnaryExpr {
	return &ast.UnaryExpr{
		OpPos: DeserializePos(pass, input.OpPos),
		Op:    tokens[input.Op],
		X:     DeserializeExpr(pass, input.X),
	}
}

func DeserializeBinaryExpr(pass *dePass, input *BinaryExpr) *ast.BinaryExpr {
	return &ast.BinaryExpr{
		X:     DeserializeExpr(pass, input.X),
		OpPos: DeserializePos(pass, input.OpPos),
		Op:    tokens[input.Op],
		Y:     DeserializeExpr(pass, input.Y),
	}
}

func DeserializeKeyValueExpr(pass *dePass, input *KeyValueExpr) *ast.KeyValueExpr {
	return &ast.KeyValueExpr{
		Key:   DeserializeExpr(pass, input.Key),
		Colon: DeserializePos(pass, input.Colon),
		Value: DeserializeExpr(pass, input.Value),
	}
}

// ----------------- Types ----------------- //

func DeserializeArrayType(pass *dePass, input *ArrayType) *ast.ArrayType {
	return &ast.ArrayType{
		Lbrack: DeserializePos(pass, input.Lbrack),
		Len:    DeserializeExpr(pass, input.Len),
		Elt:    DeserializeExpr(pass, input.Elt),
	}
}

func DeserializeStructType(pass *dePass, input *StructType) *ast.StructType {
	return &ast.StructType{
		Struct:     DeserializePos(pass, input.Struct),
		Fields:     DeserializeFieldList(pass, input.Fields),
		Incomplete: input.Incomplete,
	}
}

func DeserializeFuncType(pass *dePass, input *FuncType) *ast.FuncType {
	return &ast.FuncType{
		Func:       DeserializePos(pass, input.Func),
		TypeParams: DeserializeFieldList(pass, input.TypeParams),
		Params:     DeserializeFieldList(pass, input.Params),
		Results:    DeserializeFieldList(pass, input.Results),
	}
}

func DeserializeInterfaceType(pass *dePass, input *InterfaceType) *ast.InterfaceType {
	return &ast.InterfaceType{
		Interface:  DeserializePos(pass, input.Interface),
		Methods:    DeserializeFieldList(pass, input.Methods),
		Incomplete: input.Incomplete,
	}
}

func DeserializeMapType(pass *dePass, input *MapType) *ast.MapType {
	return &ast.MapType{
		Map:   DeserializePos(pass, input.Map),
		Key:   DeserializeExpr(pass, input.Key),
		Value: DeserializeExpr(pass, input.Value),
	}
}

func DeserializeChanType(pass *dePass, input *ChanType) *ast.ChanType {
	return &ast.ChanType{
		Begin: DeserializePos(pass, input.Begin),
		Arrow: DeserializePos(pass, input.Arrow),
		Dir:   ast.ChanDir(input.Dir),
		Value: DeserializeExpr(pass, input.Value),
	}
}

func DeserializeExpr(pass *dePass, expr Expr) ast.Expr {
	switch e := expr.(type) {
	case *Ident:
		return DeserializeIdent(pass, e)
	case *BasicLit:
		return DeserializeBasicLit(pass, e)
	case *CompositeLit:
		return DeserializeCompositeLit(pass, e)
	case *FuncLit:
		return DeserializeFuncLit(pass, e)
	case *Ellipsis:
		return DeserializeEllipsis(pass, e)
	case *BadExpr:
		return DeserializeBadExpr(pass, e)
	case *SelectorExpr:
		return DeserializeSelectorExpr(pass, e)
	case *IndexExpr:
		return DeserializeIndexExpr(pass, e)
	case *IndexListExpr:
		return DeserializeIndexListExpr(pass, e)
	case *SliceExpr:
		return DeserializeSliceExpr(pass, e)
	case *TypeAssertExpr:
		return DeserializeTypeAssertExpr(pass, e)
	case *CallExpr:
		return DeserializeCallExpr(pass, e)
	case *StarExpr:
		return DeserializeStarExpr(pass, e)
	case *UnaryExpr:
		return DeserializeUnaryExpr(pass, e)
	case *BinaryExpr:
		return DeserializeBinaryExpr(pass, e)
	case *KeyValueExpr:
		return DeserializeKeyValueExpr(pass, e)

	case *ArrayType:
		return DeserializeArrayType(pass, e)
	case *StructType:
		return DeserializeStructType(pass, e)
	case *FuncType:
		return DeserializeFuncType(pass, e)
	case *InterfaceType:
		return DeserializeInterfaceType(pass, e)
	case *MapType:
		return DeserializeMapType(pass, e)
	case *ChanType:
		return DeserializeChanType(pass, e)
	default:
		return nil
	}
}

// ----------------- Statements ----------------- //

func DeserializeBadStmt(pass *dePass, input *BadStmt) *ast.BadStmt {
	return &ast.BadStmt{
		From: DeserializePos(pass, input.From),
		To:   DeserializePos(pass, input.To),
	}
}

func DeserializeDeclStmt(pass *dePass, input *DeclStmt) *ast.DeclStmt {
	return &ast.DeclStmt{
		Decl: DeserializeDecl(pass, input.Decl),
	}
}

func DeserializeEmptyStmt(pass *dePass, input *EmptyStmt) *ast.EmptyStmt {
	return &ast.EmptyStmt{
		Semicolon: DeserializePos(pass, input.Semicolon),
		Implicit:  input.Implicit,
	}
}

func DeserializeLabeledStmt(pass *dePass, input *LabeledStmt) *ast.LabeledStmt {
	return &ast.LabeledStmt{
		Label: DeserializeIdent(pass, input.Label),
		Colon: DeserializePos(pass, input.Colon),
		Stmt:  DeserializeStmt(pass, input.Stmt),
	}
}

func DeserializeExprStmt(pass *dePass, input *ExprStmt) *ast.ExprStmt {
	return &ast.ExprStmt{
		X: DeserializeExpr(pass, input.X),
	}
}

func DeserializeSendStmt(pass *dePass, input *SendStmt) *ast.SendStmt {
	return &ast.SendStmt{
		Chan:  DeserializeExpr(pass, input.Chan),
		Arrow: DeserializePos(pass, input.Arrow),
		Value: DeserializeExpr(pass, input.Value),
	}
}

func DeserializeIncDecStmt(pass *dePass, input *IncDecStmt) *ast.IncDecStmt {
	return &ast.IncDecStmt{
		X: DeserializeExpr(pass, input.X),
	}
}

func DeserializeAssignStmt(pass *dePass, input *AssignStmt) *ast.AssignStmt {
	return &ast.AssignStmt{
		Lhs:    DeserializeList[Expr, ast.Expr](pass, input.Lhs, DeserializeExpr),
		TokPos: DeserializePos(pass, input.TokPos),
		Tok:    tokens[input.Tok],
		Rhs:    DeserializeList[Expr, ast.Expr](pass, input.Rhs, DeserializeExpr),
	}
}

func DeserializeGoStmt(pass *dePass, input *GoStmt) *ast.GoStmt {
	return &ast.GoStmt{
		Go:   DeserializePos(pass, input.Go),
		Call: DeserializeCallExpr(pass, input.Call),
	}
}

func DeserializeDeferStmt(pass *dePass, input *DeferStmt) *ast.DeferStmt {
	return &ast.DeferStmt{
		Defer: DeserializePos(pass, input.Defer),
		Call:  DeserializeCallExpr(pass, input.Call),
	}
}

func DeserializeBranchStmt(pass *dePass, input *BranchStmt) *ast.BranchStmt {
	return &ast.BranchStmt{
		TokPos: DeserializePos(pass, input.TokPos),
		Tok:    tokens[input.Tok],
		Label:  DeserializeIdent(pass, input.Label),
	}
}

func DeserializeReturnStmt(pass *dePass, input *ReturnStmt) *ast.ReturnStmt {
	return &ast.ReturnStmt{
		Return:  DeserializePos(pass, input.Return),
		Results: DeserializeList[Expr, ast.Expr](pass, input.Results, DeserializeExpr),
	}
}

func DeserializeBlockStmt(pass *dePass, input *BlockStmt) *ast.BlockStmt {
	return &ast.BlockStmt{
		Lbrace: DeserializePos(pass, input.Lbrace),
		List:   DeserializeList[Stmt, ast.Stmt](pass, input.List, DeserializeStmt),
		Rbrace: DeserializePos(pass, input.Rbrace),
	}
}

func DeserializeIfStmt(pass *dePass, input *IfStmt) *ast.IfStmt {
	return &ast.IfStmt{
		If:   DeserializePos(pass, input.If),
		Init: DeserializeStmt(pass, input.Init),
		Cond: DeserializeExpr(pass, input.Cond),
		Body: DeserializeBlockStmt(pass, input.Body),
		Else: DeserializeStmt(pass, input.Else),
	}
}

func DeserializeCaseClause(pass *dePass, input *CaseClause) *ast.CaseClause {
	return &ast.CaseClause{
		Case:  DeserializePos(pass, input.Case),
		List:  DeserializeList[Expr, ast.Expr](pass, input.List, DeserializeExpr),
		Colon: DeserializePos(pass, input.Colon),
		Body:  DeserializeList[Stmt, ast.Stmt](pass, input.Body, DeserializeStmt),
	}
}

func DeserializeSwitchStmt(pass *dePass, input *SwitchStmt) *ast.SwitchStmt {
	return &ast.SwitchStmt{
		Switch: DeserializePos(pass, input.Switch),
		Init:   DeserializeStmt(pass, input.Init),
		Tag:    DeserializeExpr(pass, input.Tag),
		Body:   DeserializeBlockStmt(pass, input.Body),
	}
}

func DeserializeTypeSwitchStmt(pass *dePass, input *TypeSwitchStmt) *ast.TypeSwitchStmt {
	return &ast.TypeSwitchStmt{
		Switch: DeserializePos(pass, input.Switch),
		Init:   DeserializeStmt(pass, input.Init),
		Assign: DeserializeStmt(pass, input.Assign),
		Body:   DeserializeBlockStmt(pass, input.Body),
	}
}

func DeserializeCommClause(pass *dePass, input *CommClause) *ast.CommClause {
	return &ast.CommClause{
		Case:  DeserializePos(pass, input.Case),
		Comm:  DeserializeStmt(pass, input.Comm),
		Colon: DeserializePos(pass, input.Colon),
		Body:  DeserializeList[Stmt, ast.Stmt](pass, input.Body, DeserializeStmt),
	}
}

func DeserializeSelectStmt(pass *dePass, input *SelectStmt) *ast.SelectStmt {
	return &ast.SelectStmt{
		Select: DeserializePos(pass, input.Select),
		Body:   DeserializeBlockStmt(pass, input.Body),
	}
}

func DeserializeForStmt(pass *dePass, input *ForStmt) *ast.ForStmt {
	return &ast.ForStmt{
		For:  DeserializePos(pass, input.For),
		Init: DeserializeStmt(pass, input.Init),
		Cond: DeserializeExpr(pass, input.Cond),
		Post: DeserializeStmt(pass, input.Post),
		Body: DeserializeBlockStmt(pass, input.Body),
	}
}

func DeserializeRangeStmt(pass *dePass, input *RangeStmt) *ast.RangeStmt {
	return &ast.RangeStmt{
		For:    DeserializePos(pass, input.For),
		Key:    DeserializeExpr(pass, input.Key),
		Value:  DeserializeExpr(pass, input.Value),
		TokPos: DeserializePos(pass, input.TokPos),
		Tok:    tokens[input.Tok],
		Range:  DeserializePos(pass, input.Range),
		X:      DeserializeExpr(pass, input.X),
		Body:   DeserializeBlockStmt(pass, input.Body),
	}
}

func DeserializeStmt(pass *dePass, stmt Stmt) ast.Stmt {
	switch s := stmt.(type) {
	case *BadStmt:
		return DeserializeBadStmt(pass, s)
	case *DeclStmt:
		return DeserializeDeclStmt(pass, s)
	case *EmptyStmt:
		return DeserializeEmptyStmt(pass, s)
	case *LabeledStmt:
		return DeserializeLabeledStmt(pass, s)
	case *ExprStmt:
		return DeserializeExprStmt(pass, s)
	case *IncDecStmt:
		return DeserializeIncDecStmt(pass, s)
	case *AssignStmt:
		return DeserializeAssignStmt(pass, s)
	case *GoStmt:
		return DeserializeGoStmt(pass, s)
	case *DeferStmt:
		return DeserializeDeferStmt(pass, s)
	case *ReturnStmt:
		return DeserializeReturnStmt(pass, s)
	case *BranchStmt:
		return DeserializeBranchStmt(pass, s)
	case *SendStmt:
		return DeserializeSendStmt(pass, s)
	case *IfStmt:
		return DeserializeIfStmt(pass, s)
	case *CaseClause:
		return DeserializeCaseClause(pass, s)
	case *SwitchStmt:
		return DeserializeSwitchStmt(pass, s)
	case *TypeSwitchStmt:
		return DeserializeTypeSwitchStmt(pass, s)
	case *BlockStmt:
		return DeserializeBlockStmt(pass, s)
	case *CommClause:
		return DeserializeCommClause(pass, s)
	case *SelectStmt:
		return DeserializeSelectStmt(pass, s)
	case *ForStmt:
		return DeserializeForStmt(pass, s)
	case *RangeStmt:
		return DeserializeRangeStmt(pass, s)
	default:
		return nil
	}
}

// ----------------- Specifications ----------------- //

func DeserializeImportSpec(pass *dePass, input *ImportSpec) *ast.ImportSpec {
	return &ast.ImportSpec{
		Doc:     DeserializeCommentGroup(pass, input.Doc),
		Name:    DeserializeIdent(pass, input.Name),
		Path:    DeserializeBasicLit(pass, input.Path),
		Comment: DeserializeCommentGroup(pass, input.Comment),
		EndPos:  DeserializePos(pass, input.EndPos),
	}
}

func DeserializeValueSpec(pass *dePass, input *ValueSpec) *ast.ValueSpec {
	return &ast.ValueSpec{
		Doc:     DeserializeCommentGroup(pass, input.Doc),
		Names:   DeserializeList[*Ident, *ast.Ident](pass, input.Names, DeserializeIdent),
		Type:    DeserializeExpr(pass, input.Type),
		Values:  DeserializeList[Expr, ast.Expr](pass, input.Values, DeserializeExpr),
		Comment: DeserializeCommentGroup(pass, input.Comment),
	}
}

func DeserializeTypeSpec(pass *dePass, input *TypeSpec) *ast.TypeSpec {
	return &ast.TypeSpec{
		Doc:        DeserializeCommentGroup(pass, input.Doc),
		Name:       DeserializeIdent(pass, input.Name),
		TypeParams: DeserializeFieldList(pass, input.TypeParams),
		Assign:     DeserializePos(pass, input.Assign),
		Type:       DeserializeExpr(pass, input.Type),
		Comment:    DeserializeCommentGroup(pass, input.Comment),
	}
}

func DeserializeSpec(pass *dePass, spec Spec) ast.Spec {
	switch s := spec.(type) {
	case *ImportSpec:
		return DeserializeImportSpec(pass, s)
	case *ValueSpec:
		return DeserializeValueSpec(pass, s)
	case *TypeSpec:
		return DeserializeTypeSpec(pass, s)
	default:
		return nil
	}
}

// ----------------- Declarations ----------------- //

func DeserializeBadDecl(pass *dePass, input *BadDecl) *ast.BadDecl {
	return &ast.BadDecl{
		From: DeserializePos(pass, input.From),
		To:   DeserializePos(pass, input.To),
	}
}

func DeserializeGenDecl(pass *dePass, input *GenDecl) *ast.GenDecl {
	return &ast.GenDecl{
		Doc:    DeserializeCommentGroup(pass, input.Doc),
		TokPos: DeserializePos(pass, input.TokenPos),
		Tok:    tokens[input.Tok],
		Lparen: token.NoPos,
		Specs:  DeserializeList[Spec, ast.Spec](pass, input.Specs, DeserializeSpec),
		Rparen: token.NoPos,
	}
}

func DeserializeFuncDecl(pass *dePass, input *FuncDecl) *ast.FuncDecl {
	return &ast.FuncDecl{
		Doc:  DeserializeCommentGroup(pass, input.Doc),
		Recv: DeserializeFieldList(pass, input.Recv),
		Name: DeserializeIdent(pass, input.Name),
		Type: DeserializeFuncType(pass, input.Type),
		Body: DeserializeBlockStmt(pass, input.Body),
	}
}

func DeserializeDecl(pass *dePass, decl Decl) ast.Decl {
	switch d := decl.(type) {
	case *BadDecl:
		return DeserializeBadDecl(pass, d)
	case *GenDecl:
		return DeserializeGenDecl(pass, d)
	case *FuncDecl:
		return DeserializeFuncDecl(pass, d)
	default:
		return nil
	}
}

// ----------------- Files and Packages ----------------- //

func DeserializeFile(pass *dePass, input *File) *ast.File {
	if err := processTokenFile(pass, input); err != nil {
		log.Fatal(err)
	}

	return &ast.File{
		Doc:        DeserializeOption(pass, input.Doc, DeserializeCommentGroup),
		Package:    DeserializePos(pass, input.Package),
		Name:       DeserializeOption(pass, input.Name, DeserializeIdent),
		Decls:      DeserializeList(pass, input.Decls, DeserializeDecl),
		FileStart:  DeserializePos(pass, input.FileStart),
		FileEnd:    DeserializePos(pass, input.FileEnd),
		Imports:    DeserializeList(pass, input.Imports, DeserializeImportSpec),
		Unresolved: DeserializeList(pass, input.Unresolved, DeserializeIdent),
		Comments:   DeserializeList(pass, input.Comments, DeserializeCommentGroup),
		GoVersion:  input.GoVersion,
	}
}

// func processTokenFile(pass *DePass, input *File) error {
// 	if pass.fset != nil && input.Name != nil {
// 		pos, ok := input.Name.NamePos.(*Position)
// 		if !ok {
// 			return fmt.Errorf("failed to get start pos for file `%s`", input.Name.Name)
// 		}

// 		fileSize := input.Size
// 		if fileSize <= 0 {
// 			fileSize = _GOARCH()
// 		}

// 		tokFile := pass.fset.AddFile(pos.Filename, -1, fileSize)
// 		tokFile.SetLinesForContent([]byte{})
// 	}

// 	return nil
// }

func processTokenFile(pass *dePass, input *File) error {
	if input == nil || input.Name == nil {
		return fmt.Errorf("input or input.Name is nil")
	}

	pos, ok := input.Name.NamePos.(*Position)
	if !ok {
		// pos := &Position{0, 0, 0}
		return fmt.Errorf("failed to get start pos for file `%s`", input.Name.Name)
	}

	fileSize := input.Size
	if fileSize <= 0 {
		fileSize = _GOARCH()
	}

	tokFile := pass.fset.AddFile(pos.Filename(), pos.Offset(), fileSize)
	if tokFile == nil {
		return fmt.Errorf("failed to add file to file set")
	}

	tokFile.SetLinesForContent([]byte{})
	return nil
}
