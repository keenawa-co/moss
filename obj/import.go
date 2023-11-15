package obj

import (
	"go/ast"
)

type ImpKind int

const (
	ImportTypeExternal ImpKind = iota
	ImportTypeInternal
	ImportTypeSideEffect
)

type ImportObj struct {
	Path       string
	Name       *IdentObj
	ImportKind ImpKind
	TypeKind   ObjKind
}

func (o *ImportObj) Kind() ObjKind {
	return o.TypeKind
}

func (o *ImportObj) IsValid() bool {
	return o.Path != ""
}

func (o *ImportObj) IsExported() bool {
	return false
}

func NewImportObj(importSpec *ast.ImportSpec, kind ImpKind) *ImportObj {
	importObj := new(ImportObj)

	if importSpec.Name != nil {
		importObj.Name = NewIdentObj(importSpec.Name)
	}

	importObj.Path = importSpec.Path.Value
	importObj.ImportKind = kind

	return importObj
}
