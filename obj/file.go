package obj

import (
	"fmt"
	"go/token"
	"path"
	"reflect"
)

type importTree struct {
	Internal   []string
	External   []string
	SideEffect []string
	Cache      map[string]int
}

type FileObj struct {
	FileSet *token.FileSet

	Name    *IdentObj
	Imports *importTree
	Types   []*TypeObj
	Decls   []*DeclObj
}

func (o *FileObj) Save(object Object) error {

	switch obj := object.(type) {
	case *ImportObj:
		o.AppendImport(obj)
		return nil

	case *TypeObj:
		o.AppendType(obj)
		return nil

	case *DeclObj:
		o.AppendDecl(obj)
		return nil

	default:
		return fmt.Errorf("%s: invalid object type", reflect.TypeOf(obj))
	}
}

func (o *FileObj) AppendType(typ *TypeObj) {
	if o.Types == nil {
		o.Types = make([]*TypeObj, 0)
	}

	o.Types = append(o.Types, typ)

}

func (o *FileObj) AppendDecl(decl *DeclObj) {
	if o.Decls == nil {
		o.Decls = make([]*DeclObj, 0)
	}

	o.Decls = append(o.Decls, decl)
}

func (o *FileObj) AppendImport(object *ImportObj) {
	switch object.ImportKind {
	case Internal:
		var importName string
		if object.Name == nil {
			importName = path.Base(object.Path)
		}

		o.Imports.Cache[importName] = len(o.Imports.Internal)
		o.Imports.Internal = append(o.Imports.Internal, object.Path)
	case External:
		o.Imports.External = append(o.Imports.External, object.Path)
	case SideEffect:
		o.Imports.SideEffect = append(o.Imports.SideEffect, object.Path)
	}
}

func NewFileObj(fset *token.FileSet, moduleName, fileName string) *FileObj {
	return &FileObj{
		Name: &IdentObj{
			Name: fileName,
			Kind: Fle,
		},
		FileSet: fset,
		Imports: &importTree{
			Internal:   make([]string, 0),
			External:   make([]string, 0),
			SideEffect: make([]string, 0),
			Cache:      make(map[string]int),
		},
	}
}
