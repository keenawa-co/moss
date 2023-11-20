package compass

import (
	"fmt"
	"go/ast"
	"go/parser"
	"go/token"
	"path/filepath"
	"sync"

	"github.com/4rchr4y/go-compass/obj"
	"golang.org/x/mod/modfile"
)

type Engine struct {
	noCopy    noCopy
	noCompare noCompare

	MaxEngineConcurrency uint

	// Required to identify internal dependencies.
	modfile *modfile.File

	// group is a map of functions of analyzer creators. The map key is the analyzer key,
	// which is identical to the key for the already created analyzer in the map of analyzers.
	// Used to create a map of analyzers for each visitor.
	group PickerFactoryGroup
}

func (e *Engine) ParseDir(targetDir string) ([]*obj.PackageObj, error) {
	fset := token.NewFileSet()
	pkg, err := parser.ParseDir(fset, targetDir, nil, parser.AllErrors)
	if err != nil {
		return nil, err
	}

	return e.processPkgGroup(fset, pkg, targetDir), nil
}

func (e *Engine) processPkgGroup(fset *token.FileSet, pkg map[string]*ast.Package, targetDir string) []*obj.PackageObj {
	resultsChan := make(chan *obj.PackageObj, len(pkg))
	var wg sync.WaitGroup

	for _, pkgAst := range pkg {
		wg.Add(1)
		go func(pa *ast.Package) {
			defer wg.Done()

			resultsChan <- e.processPkg(fset, pa, targetDir)
		}(pkgAst)
	}

	go func() {
		wg.Wait()
		close(resultsChan)
	}()

	var results []*obj.PackageObj
	fmt.Println(len(pkg))
	for r := range resultsChan {
		results = append(results, r)
	}

	return results
}

func (e *Engine) processPkg(fset *token.FileSet, pkgAst *ast.Package, targetDir string) *obj.PackageObj {
	fileObjChan := make(chan *obj.FileObj, len(pkgAst.Files))
	pkgObj := obj.NewPackageObj(pkgAst, targetDir)

	var wg sync.WaitGroup
	sema := make(chan struct{}, e.MaxEngineConcurrency)

	for fileName, fileAst := range pkgAst.Files {
		wg.Add(1)
		sema <- struct{}{}

		go func(fileAst *ast.File, fileName string) {
			defer wg.Done()
			fileObj := e.processFile(fset, fileAst, fileName)
			fileObjChan <- fileObj
			<-sema
		}(fileAst, fileName)
		sema <- struct{}{}
	}

	go func() {
		wg.Wait()
		close(fileObjChan)
	}()

	for fileObj := range fileObjChan {
		pkgObj.AppendFile(fileObj)
	}

	return pkgObj
}

func (e *Engine) processFile(fset *token.FileSet, fileAst *ast.File, fileName string) *obj.FileObj {
	fileObj := obj.NewFileObj(fset, e.modfile.Module.Mod.Path, filepath.Base(fileName))
	visitor := NewVisitor(e.group)
	state := &State{
		File:    fileObj,
		Modfile: e.modfile,
	}

	walk(state, visitor, fileAst)
	return fileObj
}
