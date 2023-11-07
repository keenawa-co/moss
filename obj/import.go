package obj

import (
	"go/ast"
	"strings"
)

type ImportType int

const (
	ImportTypeExternal ImportType = iota
	ImportTypeInternal
	ImportTypeSideEffect
)

type ImportObj struct {
	Path       string
	Alias      string
	WithAlias  bool
	ImportType ImportType
}

func (o ImportObj) Type() string {
	return "import"
}

func (o *ImportObj) CheckAndTrim(modName string) bool {
	if len(o.Path) < len(modName) {
		return false
	}

	if !strings.HasPrefix(o.Path, modName) {
		return false
	}

	o.Path = o.Path[len(modName):]
	return true
}

func NewImportObj(importSpec *ast.ImportSpec, typ ImportType) *ImportObj {
	var isWithAlias bool
	var alias string

	if importSpec.Name != nil {
		alias = importSpec.Name.Name
		isWithAlias = true
	}
	return &ImportObj{
		Path:       strings.Trim(importSpec.Path.Value, `"`),
		Alias:      alias,
		WithAlias:  isWithAlias,
		ImportType: typ,
	}
}
