package obj

import (
	"go/ast"
	"sync"
)

type PackageObj struct {
	mutex sync.Mutex

	Name  *IdentObj // package name
	Path  string    // file system path where the package is located
	Files []*FileObj
}

func NewPackageObj(pkgAst *ast.Package, path string) *PackageObj {
	return &PackageObj{
		Name: &IdentObj{
			Name: pkgAst.Name,
			Kind: Pkg,
		},
		Path: path,
	}
}
func (o *PackageObj) AppendFile(obj *FileObj) {
	o.mutex.Lock()

	if o.Files == nil {
		o.Files = make([]*FileObj, 0)
	}

	o.Files = append(o.Files, obj)
	o.mutex.Unlock()
}
