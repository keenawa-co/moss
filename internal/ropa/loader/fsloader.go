package loader

import (
	"archive/tar"
	"compress/gzip"
	"fmt"
	"io"
	"os"
	"path/filepath"

	"github.com/4rchr4y/goray/internal/ropa/types"
	"github.com/4rchr4y/goray/pkg/gvalidate"
	"github.com/open-policy-agent/opa/ast"
)

type osWrapper interface {
	Open(name string) (*os.File, error)
	GzipReader(reader io.Reader) (*gzip.Reader, error)
	TarReader(reader io.Reader) *tar.Reader
	ReadFile(name string) ([]byte, error)
}

type ioWrapper interface {
	ReadAll(reader io.Reader) ([]byte, error)
}

type bundleProcessor interface {
	Parse(input *ParseInput) (*types.Bundle, error)
}

type FsLoader struct {
	osWrap  osWrapper
	ioWrap  ioWrapper
	bParser bundleProcessor
}

type FsLoaderConf struct {
	OsWrap      osWrapper
	IoWrap      ioWrapper
	TomlDecoder bpTOMLDecoder
}

func NewFsLoader(conf *FsLoaderConf) *FsLoader {
	return &FsLoader{
		osWrap: conf.OsWrap,
		ioWrap: conf.IoWrap,
		bParser: &BundleParser{
			decoder: conf.TomlDecoder,
		},
	}
}

func (loader *FsLoader) LoadRegoFile(path string) (*types.RawRegoFile, error) {
	content, err := loader.osWrap.ReadFile(path)
	if err != nil {
		return nil, fmt.Errorf("error opening file: %v", err)
	}

	parsed, err := ast.ParseModule(path, string(content))
	if err != nil {
		return nil, fmt.Errorf("error parsing file contents: %w", err)
	}

	return &types.RawRegoFile{
		Path:   path,
		Raw:    content,
		Parsed: parsed,
	}, nil
}

func (loader *FsLoader) LoadBundle(path string) (*types.Bundle, error) {
	file, err := loader.osWrap.Open(path)
	if err != nil {
		return nil, fmt.Errorf("error opening file: %v", err)
	}
	defer file.Close()

	gr, err := loader.osWrap.GzipReader(file)
	if err != nil {
		return nil, fmt.Errorf("error creating gzip reader: %v", err)
	}
	defer gr.Close()

	files, err := loader.extractTarContent(loader.osWrap.TarReader(gr))
	if err != nil {
		return nil, fmt.Errorf("error extracting tar content: %v", err)
	}

	return loader.bParser.Parse(&ParseInput{
		FileName: path,
		Files:    files,
	})
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

		buf, err := loader.ioWrap.ReadAll(tr)
		if err != nil {
			return nil, fmt.Errorf("error reading file contents: %v", err)
		}

		result[cleanName] = buf
	}

	return result, nil
}
