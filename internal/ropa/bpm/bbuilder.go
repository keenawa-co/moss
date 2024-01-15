package bpm

import (
	"fmt"
	"io"
	"os"
	"path/filepath"
	"strings"

	"github.com/4rchr4y/goray/constant"
	"github.com/4rchr4y/goray/internal/ropa/types"
	"github.com/4rchr4y/goray/internal/ropa/utils"
	"github.com/4rchr4y/goray/version"
)

type bbOsWrapper interface {
	Walk(root string, fn filepath.WalkFunc) error
}

type tarCompressor interface {
	Compress(dirPath string, targetDir string, archiveName string) error
}

type regoFileLoader interface {
	LoadRegoFile(path string) (*types.RawRegoFile, error)
}

type BundleBuilder struct {
	osWrap     bbOsWrapper
	compressor tarCompressor
	coder      tomlCoder
	loader     regoFileLoader
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
		return fmt.Errorf("failed to create %s/%s: %v", input.SourcePath, constant.BPMLockFile, err)
	}

	if err := utils.EncodeBPMFile(bb.coder, input.BLWriter, bundlelock); err != nil {
		return fmt.Errorf("error occurred while writing to %s/%s: %v", input.SourcePath, constant.BPMLockFile, err)
	}

	return bb.buildBundle(input)
}

func (bb *BundleBuilder) createBundleLockFile(dirPath string) (*types.BundleLockFile, error) {
	modules := make([]*types.ModuleDef, 0)

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

	err := bb.osWrap.Walk(dirPath, walkFunc)
	if err != nil {
		return nil, fmt.Errorf("error walking the path %s: %v", dirPath, err)
	}

	return &types.BundleLockFile{
		Version: version.BPM,
		Modules: modules,
	}, nil
}

func (bb *BundleBuilder) processRegoFile(path string) (*types.ModuleDef, error) {
	rawRegoFile, err := bb.loader.LoadRegoFile(path)
	if err != nil {
		return nil, err
	}

	modDef := &types.ModuleDef{
		Name:         rawRegoFile.Parsed.Package.Path.String(),
		Source:       path,
		Checksum:     rawRegoFile.Sum(),
		Dependencies: getImportsList(rawRegoFile),
	}

	return modDef, nil
}

func (bb *BundleBuilder) buildBundle(input *BundleBuildInput) error {
	if err := bb.compressor.Compress(input.SourcePath, input.DestPath, input.BundleName); err != nil {
		return fmt.Errorf("error occurred while building '%s' bundle: %v", input.BundleName, err)
	}

	return nil
}

func getImportsList(file *types.RawRegoFile) []string {
	result := make([]string, len(file.Parsed.Imports))

	for i, _import := range file.Parsed.Imports {
		result[i] = _import.Path.String()
	}

	return result
}
