package loader

import (
	"archive/tar"
	"compress/gzip"
	"fmt"
	"io"
	"io/fs"
	"os"
	"path/filepath"

	"github.com/4rchr4y/goray/constant"
	"github.com/4rchr4y/goray/internal/ropa/types"
	"github.com/4rchr4y/goray/pkg/gvalidate"
	"github.com/open-policy-agent/opa/ast"
)

type PathType int

const (
	Unknown PathType = iota
	RegoFile
	DataFile
	Dir
	TarGzArchive
)

type fsWrapper interface {
	OpenFile(name string) (*os.File, error)
	GzipReader(reader io.Reader) (*gzip.Reader, error)
	TarReader(reader io.Reader) *tar.Reader
	ReadFile(name string) ([]byte, error)
	ReadAll(reader io.Reader) ([]byte, error)
}

type osWrapper interface {
	Open(name string) (*os.File, error)
}

type ioWrapper interface {
	ReadAll(r io.Reader) ([]byte, error)
}

type tomlDecoder interface {
	Decode(data string, v interface{}) error
}

type FsLoader struct {
	fs   fsWrapper
	os   osWrapper
	io   ioWrapper
	toml tomlDecoder
}

type LoaderInput struct {
	Paths  []string
	Filter func(path string, info fs.FileInfo) error
}

type LoaderResult struct {
	RegoFiles map[string]*types.RawRegoFile
	Bundles   map[string]*types.Bundle
}

func NewFsLoader(fs fsWrapper) *FsLoader {
	return &FsLoader{
		fs: fs,
	}
}

func (loader *FsLoader) LoadRegoFile(path string) (*types.RawRegoFile, error) {
	content, err := loader.fs.ReadFile(path)
	if err != nil {
		return nil, fmt.Errorf("error opening file: %v", err)
	}

	parsed, err := ast.ParseModule(path, string(content))
	if err != nil {
		return nil, fmt.Errorf("error parsing file contents: %w", err)
	}

	return &types.RawRegoFile{
		Path:   path,
		Parsed: parsed,
	}, nil
}

func (loader *FsLoader) LoadBundle(path string) (*types.Bundle, error) {
	file, err := loader.fs.OpenFile(path)
	if err != nil {
		return nil, fmt.Errorf("error opening file: %v", err)
	}
	defer file.Close()

	gr, err := loader.fs.GzipReader(file)
	if err != nil {
		return nil, fmt.Errorf("error creating gzip reader: %v", err)
	}
	defer gr.Close()

	files, err := loader.extractTarContent(loader.fs.TarReader(gr))
	if err != nil {
		return nil, fmt.Errorf("error extracting tar content: %v", err)
	}

	// TODO: move process logic to a separate service
	return loader.processBundleFiles(files, path)
}

func (loader *FsLoader) processBundleFiles(files map[string][]byte, bundlePath string) (*types.Bundle, error) {
	bundle := &types.Bundle{
		Name:      filepath.Clean(bundlePath),
		RegoFiles: make(map[string]*types.RawRegoFile),
	}

	for filePath, content := range files {
		switch {
		case isRegoFile(filePath):
			parsed, err := loader.processRegoFile(bundle, content, filePath)
			if err != nil {
				return nil, err
			}

			bundle.RegoFiles[filePath] = &types.RawRegoFile{
				Path:   filePath,
				Parsed: parsed,
			}

		case isBPMFile(filePath):
			bundlefile, err := loader.processBPMFile(content)
			if err != nil {
				return nil, err
			}

			bundle.BundleFile = bundlefile

		case isBPMLockFile(filePath):
			bundlelock, err := loader.processBPMLockFile(content)
			if err != nil {
				return nil, err
			}

			bundle.BundleLockFile = bundlelock

		case isBPMWorkFile(filePath):
			bpmwork, err := loader.processBPMWorkFile(content)
			if err != nil {
				return nil, err
			}

			bundle.BpmWorkFile = bpmwork
		}
	}

	return bundle, nil
}

func (loader *FsLoader) processRegoFile(bundle *types.Bundle, content []byte, filePath string) (*ast.Module, error) {
	parsed, err := ast.ParseModule(filePath, string(content))
	if err != nil {
		return nil, fmt.Errorf("error parsing file contents: %v", err)
	}

	return parsed, nil
}

func (loader *FsLoader) processBPMWorkFile(fileContent []byte) (*types.BpmWorkFile, error) {
	var bpmwork types.BpmWorkFile
	if err := loader.toml.Decode(string(fileContent), &bpmwork); err != nil {
		return nil, fmt.Errorf("error parsing bpm.work content: %v", err)
	}

	return &bpmwork, nil
}

func (loader *FsLoader) processBPMLockFile(fileContent []byte) (*types.BundleLockFile, error) {
	var bundlelock types.BundleLockFile
	if err := loader.toml.Decode(string(fileContent), &bundlelock); err != nil {
		return nil, fmt.Errorf("error parsing bundle.lock content: %v", err)
	}

	return &bundlelock, nil
}

func (loader *FsLoader) processBPMFile(fileContent []byte) (*types.BundleFile, error) {
	var bundlefile types.BundleFile
	if err := loader.toml.Decode(string(fileContent), &bundlefile); err != nil {
		return nil, fmt.Errorf("error parsing bundle.toml content: %v", err)
	}

	return &bundlefile, nil
}

func (loader *FsLoader) extractTarContent(tr *tar.Reader) (map[string][]byte, error) {
	result := make(map[string][]byte)

	for {
		header, err := tr.Next()
		if err == io.EOF {
			break
		}

		if err != nil {
			return nil, fmt.Errorf("error reading tar entry: %v", err)
		}

		if header.Typeflag != tar.TypeReg {
			continue
		}

		cleanName := filepath.Clean(header.Name)
		if err := gvalidate.ValidatePath(cleanName); err != nil {
			return nil, fmt.Errorf("path '%s' is not valid: %v", header.Name, err)
		}

		buf, err := loader.fs.ReadAll(tr)
		if err != nil {
			return nil, fmt.Errorf("error reading file contents: %v", err)
		}

		result[cleanName] = buf
	}

	return result, nil
}

func isRegoFile(filePath string) bool    { return filepath.Ext(filePath) == constant.RegoExt }
func isBPMFile(filePath string) bool     { return filePath == constant.BPMFile }
func isBPMLockFile(filePath string) bool { return filePath == constant.BPMLockFile }
func isBPMWorkFile(filePath string) bool { return filePath == constant.BPMWorkFile }
