package bpm

import (
	"fmt"
	"io"
	"os"
	"path/filepath"
	"strings"

	"github.com/4rchr4y/goray/internal/ropa/loader"
)

type fsWrapper interface {
	Walk(root string, fn filepath.WalkFunc) error
}

type tarClient interface {
	Compress(dirPath string, targetDir string, archiveName string) error
}

type regoFileLoader interface {
	LoadRegoFile(path string) (*loader.RawRegoFile, error)
}

type BundleBuilder struct {
	fswrap fsWrapper
	tar    tarClient
	toml   tomlClient
	loader regoFileLoader
}

type BundleBuildInput struct {
	_          [0]int
	SourcePath string
	DestPath   string
	BundleName string
	BLWriter   io.Writer
}

func (bb *BundleBuilder) Build(input *BundleBuildInput) error {
	bundlelock, err := bb.createBundleLockFile(input.SourcePath)
	if err != nil {
		return fmt.Errorf("failed to create %s/bundle.lock: %v", input.SourcePath, err)
	}

	if err := EncodeBundleLock(bb.toml, input.BLWriter, bundlelock); err != nil {
		return fmt.Errorf("error occurred while writing to %s/bundle.lock: %v", input.SourcePath, err)
	}

	return bb.buildBundle(input)
}

func (bb *BundleBuilder) createBundleLockFile(dirPath string) (*BundleLock, error) {
	modules := make([]*ModuleDef, 0)

	walkFunc := func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return fmt.Errorf("error occurred while accessing a path %s: %v", path, err)
		}

		if !info.IsDir() && strings.HasSuffix(info.Name(), ".rego") {
			module, err := bb.processRegoFile(path)
			if err != nil {
				return fmt.Errorf("failed to process '%s' file: %v", path, err)
			}

			modules = append(modules, module)
		}
		return nil
	}

	err := bb.fswrap.Walk(dirPath, walkFunc)
	if err != nil {
		return nil, fmt.Errorf("error walking the path %s: %v", dirPath, err)
	}

	return &BundleLock{
		Modules: modules,
	}, nil
}

func (bb *BundleBuilder) processRegoFile(path string) (*ModuleDef, error) {
	rawRegoFile, err := bb.loader.LoadRegoFile(path)
	if err != nil {
		return nil, err
	}

	modDef := &ModuleDef{
		Name:         rawRegoFile.Parsed.Package.Path.String(),
		Source:       path,
		Dependencies: getImportsList(rawRegoFile),
	}

	return modDef, nil
}

func (bb *BundleBuilder) buildBundle(input *BundleBuildInput) error {
	if err := bb.tar.Compress(input.SourcePath, input.DestPath, input.BundleName); err != nil {
		return fmt.Errorf("error occurred while building '%s' bundle: %v", input.BundleName, err)
	}

	return nil
}

func getImportsList(file *loader.RawRegoFile) []string {
	result := make([]string, len(file.Parsed.Imports))

	for i, _import := range file.Parsed.Imports {
		result[i] = _import.Path.String()
	}

	return result
}
