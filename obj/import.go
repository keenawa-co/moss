package obj

import (
	"go/ast"
	"go/token"
)

type ImportType int

const (
	ImportTypeExternal ImportType = iota
	ImportTypeInternal
	ImportTypeSideEffect
)

type ImportObj struct {
	Pos token.Pos
	End token.Pos

	Path       string
	Alias      string // TODO: should be a Name and type Ident
	WithAlias  bool
	ImportType ImportType
}

func (o *ImportObj) IsValid() bool {
	return o.Pos != token.NoPos
}

func (o *ImportObj) IsExported() bool {
	return false
}

func NewImportObj(importSpec *ast.ImportSpec, typ ImportType) *ImportObj {
	importObj := new(ImportObj)

	if importSpec.Name != nil {
		importObj.Alias = importSpec.Name.Name
		importObj.WithAlias = true
	}

	importObj.Path = importSpec.Path.Value
	importObj.ImportType = typ

	return importObj
}
