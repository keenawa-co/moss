package ropa

import (
	"fmt"
)

type repoCli interface {
	Store(file *IndexedRegoFile) error
	Load(path string) (*IndexedRegoFile, error)
}

type radixTree interface {
	Store(path []byte, value *IndexedRegoFile) (*IndexedRegoFile, bool)
	Load(path []byte) (value *IndexedRegoFile, found bool)
	LoadPrefix(prefix []byte) (map[string]*IndexedRegoFile, bool)
}

// func (o *LoadingOutput) merge() []*regom.RegoFile {
// 	result := make([]*regom.RegoFile, len(o.Dependencies)+1)
// 	result[0] = o.Parent

// 	for i := 1; i < len(o.Dependencies); i++ {
// 		result[i] = o.Dependencies[i]
// 	}

// 	return result
// }

type LoaderFn func(pkgPath string) (*IndexedRegoFile, error)

type IndexedRegoFile struct {
	*RawRegoFile
	WantList []string
}

type LinkedRegoFile struct {
	*RawRegoFile
	Dependencies map[string]*IndexedRegoFile
}

type storage struct {
	repo      repoCli
	pathCache map[string]string   // package name -> file path
	loaded    radixTree           // file path -> file
	wantList  map[string]struct{} // package name -> empty struct
}

func (s *storage) cache(file *RawRegoFile) {
	packagePath := file.Parsed.Package.Path.String()
	s.pathCache[packagePath] = file.Path
}

func (s *storage) lookup(importPath string) (string, bool) {
	filepath, ok := s.pathCache[importPath]
	return filepath, ok
}

type Linker struct {
	storage *storage
}

func NewLinker(db repoCli, rdxTree radixTree) *Linker {
	return &Linker{
		storage: &storage{
			repo:      db,
			pathCache: make(map[string]string),
			loaded:    rdxTree,
			wantList:  make(map[string]struct{}),
		},
	}
}

func (linker *Linker) Load(importPath string) (*IndexedRegoFile, error) {
	filePath, ok := linker.storage.lookup(importPath)
	if !ok {
		return nil, fmt.Errorf("import '%s' is undefined", importPath)
	}

	if file, loaded := linker.storage.loaded.Load([]byte(filePath)); loaded {
		return file, nil
	}

	file, err := linker.storage.repo.Load(filePath)
	if err != nil {
		return nil, err
	}

	return file, nil
}

func (linker *Linker) Indexing(rawFile *RawRegoFile) (*IndexedRegoFile, error) {
	linker.storage.cache(rawFile)

	wantList := make([]string, len(rawFile.Parsed.Imports))

	for i := range rawFile.Parsed.Imports {
		wantList[i] = rawFile.Parsed.Imports[i].Path.String()
	}

	indexedFile := &IndexedRegoFile{
		RawRegoFile: rawFile,
		WantList:    wantList,
	}

	if err := linker.storage.repo.Store(indexedFile); err != nil {
		return nil, err
	}

	return indexedFile, nil
}

func (linker *Linker) Linking(file *IndexedRegoFile) (*LinkedRegoFile, error) {
	deps := make(map[string]*IndexedRegoFile)

	for i := range file.WantList {
		indexedFile, err := linker.Load(file.WantList[i])
		if err != nil {
			return nil, err
		}

		deps[indexedFile.Path] = indexedFile
	}

	return &LinkedRegoFile{
		RawRegoFile:  file.RawRegoFile,
		Dependencies: deps,
	}, nil
}

// func (lkr *Linker) ProcessRegoFileMeta(file *regom.RegoFile) (*regom.RegoFileMeta, error) {
// 	deps := make([]regom.Path, len(file.Parsed.Imports))
// 	// fmt.Println(file.Parsed.Package.Path.String())

// 	fmt.Println("lookup", lkr.cache)
// 	for i, imp := range file.Parsed.Imports {
// 		path, ok := lkr.cache[imp.Path.String()]
// 		if !ok {
// 			return nil, fmt.Errorf("import '%s' is undefined", imp.Path.String())
// 		}

// 		deps[i] = regom.Path{
// 			Path: path,
// 			Type: regom.File,
// 		}
// 	}

// 	return &regom.RegoFileMeta{
// 		Dependencies: deps,
// 	}, nil
// }
