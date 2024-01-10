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
	pathCache map[string]string // package name -> file path
	loaded    radixTree         // file path -> file
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
		},
	}
}

func (linker *Linker) Loading(importPath string) (*IndexedRegoFile, error) {
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
		indexedFile, err := linker.Loading(file.WantList[i])
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
