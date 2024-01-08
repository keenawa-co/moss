package ropa

import (
	"fmt"

	"github.com/4rchr4y/goray/internal/ropa/regom"
)

type storageCli interface {
	Load(path string) (*regom.RegoFile, error)
}

type radixTree interface {
	Store(path []byte, value *LinkerOutput) (*LinkerOutput, bool)
	Load(path []byte) (value *LinkerOutput, found bool)
	LoadPrefix(prefix []byte) (map[string]*LinkerOutput, bool)
}

type LinkerInput struct {
	Parent  *regom.RegoFile
	Imports []regom.Path
}

type LinkerOutput struct {
	Parent  *regom.RegoFile
	Imports []*regom.RegoFile
}

func (o *LinkerOutput) merge() []*regom.RegoFile {
	result := make([]*regom.RegoFile, len(o.Imports)+1)
	result[0] = o.Parent

	for i := 1; i < len(o.Imports); i++ {
		result[i] = o.Imports[i]
	}

	return result
}

type Linker struct {
	storage storageCli
	loaded  radixTree         // file path -> file
	cache   map[string]string // pkg name -> file path
}

func NewLinker(db storageCli, rdxTree radixTree) *Linker {
	return &Linker{
		storage: db,
		loaded:  rdxTree,
		cache:   make(map[string]string),
	}
}

// func (lkr *Linker) load(path regom.Path) ([]*regom.RegoFile, error) {
// 	switch path.Type {
// 	case regom.Dir:
// 		return lkr.loadDir(path.Path)

// 	case regom.File:
// 		return lkr.loadFile(path.Path)

// 	default:
// 		return nil, fmt.Errorf("'%s' have invalid dependency type", path.Path)
// 	}
// }

func (lkr *Linker) loadFile(path string) (*LinkerInput, error) {
	// if set, ok := lkr.loaded.Load([]byte(path)); ok {
	// 	return set.merge(), nil
	// }

	file, err := lkr.storage.Load(path)
	if err != nil {
		return nil, err
	}
	if file == nil {
		return nil, fmt.Errorf("file %s is undefined", path)
	}

	meta, err := lkr.ProcessRegoFileMeta(file)
	if err != nil {
		return nil, err
	}

	// depsFiles := make([]*regom.RegoFile, 0)

	// for i := range imports {
	// 	files, err := lkr.loadFile(imports[i])
	// 	if err != nil {
	// 		return nil, err
	// 	}

	// 	depsFiles = append(depsFiles, files...)
	// }

	// set := &LinkerOutput{
	// 	Parent:  file,
	// 	Imports: depsFiles,
	// }

	// lkr.loaded.Store([]byte(path), set)

	return &LinkerInput{
		Parent:  file,
		Imports: meta.Dependencies,
	}, nil
}

func (lkr *Linker) loadDir(path string) ([]*regom.RegoFile, error) {
	files, ok := lkr.loaded.LoadPrefix([]byte(path))
	if !ok {
		result := make([]*regom.RegoFile, 0)

		for _, set := range files {
			result = append(result, set.merge()...)
		}

		return result, nil
	}

	panic("loadDir is not implemented")
}

func (lkr *Linker) Link(input *LinkerInput) (*LinkerOutput, error) {
	// save in the linker cache
	lkr.cache[input.Parent.Parsed.Package.Path.String()] = input.Parent.Path

	fmt.Println("saving", lkr.cache)

	imports := make([]*regom.RegoFile, 0)

	// loadedImports := make([]*regom.RegoFile, 0)

	for i := range input.Imports {
		imp := input.Imports[i]

		fmt.Println(imp.Path)
		// files, err := lkr.loadFile(input.Imports[i])
		// if err != nil {
		// 	return nil, err
		// }

		// loadedImports = append(loadedImports, files...)

		switch imp.Type {
		case regom.Dir:
			// return lkr.loadDir(input.Imports[i].Path)
		case regom.File:
			if output, ok := lkr.loaded.Load([]byte(imp.Path)); ok {
				imports = append(imports, output.merge()...)
				continue
			}

			input, err := lkr.loadFile(imp.Path)
			if err != nil {
				return nil, err
			}

			output, err := lkr.Link(input)
			if err != nil {
				return nil, err
			}

			imports = append(imports, output.merge()...)
			continue

		default:
			return nil, fmt.Errorf("'%s' have invalid dependency type", imp.Path)
		}
	}

	return &LinkerOutput{
		Parent:  input.Parent,
		Imports: imports,
	}, nil
}

// func (lkr *Linker) getImports(imports []*ast.Import) ([]regom.Path, error) {
// 	result := make([]regom.Path, len(imports))

// 	for i, imp := range imports {
// 		path, ok := lkr.cache[imp.Path.String()]
// 		if !ok {
// 			return nil, fmt.Errorf("import %s is undefined", imp.Path.String())
// 		}

// 		result[i] = regom.Path{
// 			Path: path,
// 			Type: regom.File,
// 		}
// 	}

// 	return result, nil
// }

func (lkr *Linker) ProcessRegoFileMeta(file *regom.RegoFile) (*regom.RegoFileMeta, error) {
	deps := make([]regom.Path, len(file.Parsed.Imports))
	// fmt.Println(file.Parsed.Package.Path.String())

	fmt.Println("lookup", lkr.cache)
	for i, imp := range file.Parsed.Imports {
		path, ok := lkr.cache[imp.Path.String()]
		if !ok {
			return nil, fmt.Errorf("import '%s' is undefined", imp.Path.String())
		}

		deps[i] = regom.Path{
			Path: path,
			Type: regom.File,
		}
	}

	return &regom.RegoFileMeta{
		Dependencies: deps,
	}, nil
}
