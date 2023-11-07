package obj

import (
	"go/ast"
	"sync"
)

type PackageObj struct {
	mutex sync.Mutex

	// Name holds the name of the package.
	Name string

	// Path stores the file system path where the package is located.
	Path string

	// Files is an array of FileObj pointers, each representing a file within the package.
	Files []*FileObj

	// FileIndexes maps a file name to its corresponding index in the Files array.
	// The key represents the file name, while the value corresponds to the index of the file in the `Files` array.
	FileIndexes map[string]int
}

func NewPackageObj(pkgAst *ast.Package, path string) *PackageObj {
	return &PackageObj{
		Name:        pkgAst.Name,
		Path:        path,
		FileIndexes: make(map[string]int, len(pkgAst.Files)),
	}
}
func (o *PackageObj) AppendFile(obj *FileObj) {
	o.mutex.Lock()
	o.FileIndexes[obj.Name] = len(o.Files)
	o.Files = append(o.Files, obj)
	o.mutex.Unlock()
}
