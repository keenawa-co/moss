package obj

import (
	"go/ast"
)

type PackageObj struct {
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
	if o.Files == nil {
		o.Files = make([]*FileObj, 0)
	}

	o.Files = append(o.Files, obj)
}
