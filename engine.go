package compass

import (
	"context"
	"go/ast"
	"go/parser"
	"go/token"
	"path/filepath"
	"reflect"
	"sync"

	"github.com/4rchr4y/go-compass/event"
	"github.com/4rchr4y/go-compass/obj"
)

type Engine struct {
	noCopy noCopy

	// The name of the module of the analyzed project.
	// This name must be identical to the module name in the `go.mod` file.
	// Required to identify internal dependencies.
	ModuleName string

	// Map of functions of analyzer creators. The map key is the analyzer key,
	// which is identical to the key for the already created analyzer in the map of analyzers.
	// Used to create a map of analyzers for each visitor.
	analyzerFactoryMap AnalyzerFactoryGroup

	mu   sync.RWMutex
	once sync.Once

	eventCh       chan event.Event
	unsubscribeCh chan struct{}
}

type EngineConfig struct {
	ModuleName         string
	Determinator       func(ast.Node) reflect.Type
	AnalyzerFactoryMap AnalyzerFactoryGroup
}

func NewEngine(cfg *EngineConfig) *Engine {
	engine := new(Engine)
	engine.once.Do(func() {
		engine.ModuleName = cfg.ModuleName
		engine.analyzerFactoryMap = cfg.AnalyzerFactoryMap
	})

	return engine
}

func (e *Engine) Subscribe(eventCh chan event.Event) chan struct{} {
	e.once.Do(func() {
		e.eventCh = eventCh
		e.unsubscribeCh = make(chan struct{})
	})

	return e.unsubscribeCh
}

// func (c *Compass) Run(ctx context.Context) {
// 	res := c.pipeline.Run(ctx, &plugin.CollectorPluginInput{
// 		RootDir:     ".",
// 		IgnoredList: config.DefaultIgnoredMap,
// 	})

// 	for _, dir := range res.([]string) {
// 		fmt.Println(dir)
// 	}
// }

func (e *Engine) ParseDir(targetDir string) ([]*obj.PackageObj, error) {
	fset := token.NewFileSet()
	pkg, err := parser.ParseDir(fset, targetDir, nil, parser.AllErrors)
	if err != nil {
		return nil, err
	}

	// processes each package either concurrently or sequentially based on their count
	if len(pkg) > 1 {
		return e.processMultiplePkgConcurrently(fset, pkg, targetDir), nil
	}

	return e.processSinglePkg(fset, pkg, targetDir), nil
}

// processes a single package without concurrency
func (e *Engine) processSinglePkg(fset *token.FileSet, pkg map[string]*ast.Package, targetDir string) []*obj.PackageObj {
	var results []*obj.PackageObj

	for _, pkgAst := range pkg {
		results = append(results, e.parsePkg(fset, pkgAst, targetDir))
	}

	return results
}

// processes multiple packages concurrently
func (e *Engine) processMultiplePkgConcurrently(fset *token.FileSet, pkg map[string]*ast.Package, targetDir string) []*obj.PackageObj {
	var results []*obj.PackageObj
	var wg sync.WaitGroup
	var mu sync.Mutex

	for _, pkgAst := range pkg {
		wg.Add(1)
		go func(pa *ast.Package) {
			defer wg.Done()
			pkgResult := e.parsePkg(fset, pa, targetDir)

			mu.Lock()
			results = append(results, pkgResult)
			mu.Unlock()
		}(pkgAst)
	}
	wg.Wait()

	return results
}

func (e *Engine) parsePkg(fset *token.FileSet, pkgAst *ast.Package, targetDir string) *obj.PackageObj {
	var wg sync.WaitGroup
	pkgObj := obj.NewPackageObj(pkgAst, targetDir)

	for fileName, fileAst := range pkgAst.Files {
		wg.Add(1)

		go func(fileAst *ast.File, fileName string) {
			fileObj := e.parseFile(fset, fileAst, fileName)
			pkgObj.AppendFile(fileObj)

			wg.Done()
		}(fileAst, fileName)
	}

	wg.Wait()
	return pkgObj
}

func (e *Engine) parseFile(fset *token.FileSet, fileAst *ast.File, fileName string) *obj.FileObj {
	fileObj := obj.NewFileObj(fset, e.ModuleName, filepath.Base(fileName))
	visitor := NewVisitor(visitorConfig{
		file:   fileObj,
		alzMap: e.analyzerFactoryMap.Make(fileObj), // Initializing the analyzer map
	})

	WalkWithContext(context.Background(), visitor, fileAst)
	return fileObj
}
