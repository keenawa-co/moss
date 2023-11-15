package obj

import (
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
		return fmt.Errorf("%s: invalid object type", reflect.TypeOf(obj))
	}
}

func (o *FileObj) AppendType(typ *TypeObj) {
	o.mutex.Lock()
	o.Entities.TypesIndexes[typ.Name.Name] = len(o.Entities.Types)
	o.Entities.Types = append(o.Entities.Types, typ)
	o.mutex.Unlock()
}

func (o *FileObj) AppendDecl(decl *DeclObj) {
	o.mutex.Lock()

	o.Entities.DeclIndexes[decl.Name.Name] = len(o.Entities.Decls)
	o.Entities.Decls = append(o.Entities.Decls, decl)
	o.mutex.Unlock()
}

func (o *FileObj) IsInternalDependency(name string) (int, bool) {
	index, exists := o.Entities.Imports.InternalImportsMeta[name]
	return index, exists
}

func (o *FileObj) AppendImport(object *ImportObj) {
	o.mutex.Lock()
	switch object.ImportKind {
	case Internal:
		if object.Name == nil {
			object.Name = &IdentObj{
				Name: path.Base(object.Path),
			}
		}

		o.Entities.Imports.InternalImportsMeta[object.Name.String()] = len(o.Entities.Imports.InternalImports)
		o.Entities.Imports.InternalImports = append(o.Entities.Imports.InternalImports, object.Path)
	case External:
		o.Entities.Imports.ExternalImports = append(o.Entities.Imports.ExternalImports, object.Path)
	case SideEffect:
		o.Entities.Imports.SideEffectImports = append(o.Entities.Imports.SideEffectImports, object.Path)
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
