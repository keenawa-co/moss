package obj

import (
	"go/ast"
)

type ImpKind int

const (
	External   ImpKind = iota // any package not defined within the analyzed project
	Internal                  // package defined within the analyzed project
	SideEffect                // can be either an internal or external package
)

type ImportObj struct {
	Path       string
	Name       *IdentObj
	ImportKind ImpKind
	TypeKind   ObjKind
}

func (o *ImportObj) Kind() ObjKind {
	return Imp
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
		importObj.Name = &IdentObj{
			Name: importSpec.Name.Name,
			Kind: Imp,
		}
	}

	importObj.Path = importSpec.Path.Value
	importObj.ImportKind = kind

	return importObj
}
