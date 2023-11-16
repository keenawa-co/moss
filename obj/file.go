package obj

import (
	"fmt"
	"go/token"
	"path"
	"reflect"
	"sync"
)

type importTree struct {
	Internal   []string
	External   []string
	SideEffect []string
	Meta       map[string]int
}

type FileObjEntitySet struct {
	Imports *importTree

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
	index, exists := o.Entities.Imports.Meta[name]
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

		o.Entities.Imports.Meta[object.Name.String()] = len(o.Entities.Imports.Internal)
		o.Entities.Imports.Internal = append(o.Entities.Imports.Internal, object.Path)
	case External:
		o.Entities.Imports.External = append(o.Entities.Imports.External, object.Path)
	case SideEffect:
		o.Entities.Imports.SideEffect = append(o.Entities.Imports.SideEffect, object.Path)
	}
	o.mutex.Unlock()
}

func NewFileObj(fset *token.FileSet, moduleName, fileName string) *FileObj {
	return &FileObj{
		Name:    fileName,
		FileSet: fset,
		Entities: &FileObjEntitySet{
			Imports: &importTree{
				Internal:   make([]string, 0),
				External:   make([]string, 0),
				SideEffect: make([]string, 0),
				Meta:       make(map[string]int),
			},
			Types:        make([]*TypeObj, 0),
			TypesIndexes: make(map[string]int),
			Decls:        make([]*DeclObj, 0),
			DeclIndexes:  make(map[string]int),
		},
	}
}
