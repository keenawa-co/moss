package ason

import (
	"fmt"
	"go/ast"
	"go/token"
	"log"
)

// TODO List
//
// *ast.Field
// *ast.FieldList
// *ast.BadExpr
// *ast.Ellipsis
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

type DePass struct {
	fset *token.FileSet
	ref  map[ast.Node]Ason
}

func NewDePass(fset *token.FileSet) *DePass {
	return &DePass{
		fset: fset,
	}
}

type DeFn[I Ason, R ast.Node] func(*DePass, I) R

func DeProcessList[I Ason, R ast.Node](pass *DePass, inputList []I, de DeFn[I, R]) []R {
	if inputList == nil {
		return nil
	}

	result := make([]R, len(inputList))
	for i := 0; i < len(inputList); i++ {
		result[i] = de(pass, inputList[i])
	}

	return result
}

func DeserializeFile(pass *DePass, input *File) *ast.File {
	if err := processTokenFile(pass, input); err != nil {
		log.Fatal(err)
	}

	return &ast.File{
		Name:      DeserializeIdent(pass, input.Name),
		Package:   DeserializePos(pass, input.Package),
		Decls:     DeProcessList[Decl, ast.Decl](pass, input.Decls, DeProcessDecl),
		GoVersion: input.GoVersion,
	}
}

func processTokenFile(pass *DePass, input *File) error {
	// TODO: make more simple if else condition
	if pass.fset != nil && input.Name != nil && input.Name.Loc != nil && input.Name.Loc.Start != nil {
		pos, ok := input.Name.Loc.Start.(*Position)
		if !ok {
			return fmt.Errorf("failed to get start pos for file `%s`", input.Name.Name)
		}

		tokFile := pass.fset.AddFile(pos.Filename, -1, input.Size)
		tokFile.SetLinesForContent([]byte{})
	}

	return nil
}

func DeserializePos(pass *DePass, input Pos) token.Pos {
	switch v := input.(type) {
	case *Position:
		tokFile := pass.fset.File(token.Pos(v.Offset))
		if tokFile != nil {
			return tokFile.Pos(v.Offset)
		}

		return token.NoPos

	default:
		return token.NoPos
	}
}

func DeserializeIdent(pass *DePass, input *Ident) *ast.Ident {
	return &ast.Ident{
		Name:    input.Name,
		NamePos: DeserializePos(pass, input.NamePos),
	}
}

func DeserializeBasicLit(pass *DePass, input *BasicLit) *ast.BasicLit {
	return &ast.BasicLit{
		ValuePos: DeserializePos(pass, input.ValuePos),
		Kind:     tokens[input.Kind],
		Value:    input.Value,
	}
}

func DeserializeValueSpec(pass *DePass, input *ValueSpec) *ast.ValueSpec {
	return &ast.ValueSpec{
		Names:  DeProcessList[*Ident, *ast.Ident](pass, input.Names, DeserializeIdent),
		Values: DeProcessList[Expr, ast.Expr](pass, input.Values, DeProcessExpr),
	}
}

func DeProcessExpr(pass *DePass, expr Expr) ast.Expr {
	switch e := expr.(type) {
	case *BasicLit:
		return DeserializeBasicLit(pass, e)
	default:
		return nil
	}
}

func DeserializeGenDecl(pass *DePass, input *GenDecl) *ast.GenDecl {
	return &ast.GenDecl{
		TokPos: DeserializePos(pass, input.TokenPos),
		Tok:    tokens[input.Tok],
		Lparen: token.NoPos,
		Rparen: token.NoPos,
		Specs:  DeProcessList[Spec, ast.Spec](pass, input.Specs, DeProcessSpec),
	}
}

func DeProcessSpec(pass *DePass, spec Spec) ast.Spec {
	switch s := spec.(type) {
	case *ValueSpec:
		return DeserializeValueSpec(pass, s)
	default:
		return nil
	}
}

func DeProcessDecl(pass *DePass, decl Decl) ast.Decl {
	switch d := decl.(type) {
	case *GenDecl:
		return DeserializeGenDecl(pass, d)
	default:
		return nil
	}
}
