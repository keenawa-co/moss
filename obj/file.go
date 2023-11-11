package obj

import (
	"errors"
	"fmt"
	"go/token"
	"path"
	"reflect"
	"sync"
)

type FileObjImportTree struct {
	InternalImports     []string
	ExternalImports     []string
	SideEffectImports   []string
	InternalImportsMeta map[string]int
}

type FileObjEntitySet struct {
	Imports *FileObjImportTree

	Types        []*TypeObj
	TypesIndexes map[string]int
	Decls        []*DeclObj
	DeclIndexes  map[string]int
}

type FileObj struct {
	mutex sync.Mutex

	Name     string
	FileSet  *token.FileSet
	Entities *FileObjEntitySet
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
		return errors.New(fmt.Sprintf("%s: invalid object type", reflect.TypeOf(obj)))
	}
}

func (o *FileObj) AppendType(typ *TypeObj) {
	o.mutex.Lock()
	o.Entities.TypesIndexes[typ.Name] = len(o.Entities.Types)
	o.Entities.Types = append(o.Entities.Types, typ)
	o.mutex.Unlock()
}

func (o *FileObj) AppendDecl(decl *DeclObj) {
	o.mutex.Lock()

	o.Entities.DeclIndexes[decl.Name] = len(o.Entities.Decls)
	o.Entities.Decls = append(o.Entities.Decls, decl)
	o.mutex.Unlock()
}

func (o *FileObj) IsInternalDependency(alias string) (int, bool) {
	index, exists := o.Entities.Imports.InternalImportsMeta[alias]
	return index, exists
}

func (o *FileObj) AppendImport(obj *ImportObj) {
	o.mutex.Lock()
	switch obj.ImportType {
	case ImportTypeInternal:
		alias := obj.Alias
		if !obj.WithAlias {
			alias = path.Base(obj.Path)
		}

		o.Entities.Imports.InternalImportsMeta[alias] = len(o.Entities.Imports.InternalImports)
		o.Entities.Imports.InternalImports = append(o.Entities.Imports.InternalImports, obj.Path)
	case ImportTypeExternal:
		o.Entities.Imports.ExternalImports = append(o.Entities.Imports.ExternalImports, obj.Path)
	case ImportTypeSideEffect:
		o.Entities.Imports.SideEffectImports = append(o.Entities.Imports.SideEffectImports, obj.Path)
	}
	o.mutex.Unlock()
}

func NewFileObj(fset *token.FileSet, moduleName, fileName string) *FileObj {
	return &FileObj{
		Name:    fileName,
		FileSet: fset,
		Entities: &FileObjEntitySet{
			Imports: &FileObjImportTree{
				InternalImports:     make([]string, 0),
				ExternalImports:     make([]string, 0),
				SideEffectImports:   make([]string, 0),
				InternalImportsMeta: make(map[string]int),
			},
			Types:        make([]*TypeObj, 0),
			TypesIndexes: make(map[string]int),
			Decls:        make([]*DeclObj, 0),
			DeclIndexes:  make(map[string]int),
		},
	}
}
