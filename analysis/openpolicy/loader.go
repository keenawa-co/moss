package openpolicy

import (
	"archive/tar"
	"compress/gzip"
	"fmt"
	"io"
	"io/fs"
	"os"
	"path/filepath"
	"strings"
	"unicode/utf8"

	"github.com/open-policy-agent/opa/ast"
)

type RegoFile struct {
	Path   string
	Parsed *ast.Module
}

type Bundle struct {
	Name  string
	Files []*RegoFile
}

type fsWrapper interface {
	OpenFile(name string) (*os.File, error)
	GzipReader(reader io.Reader) (*gzip.Reader, error)
	TarReader(reader io.Reader) *tar.Reader
	ReadFile(name string) ([]byte, error)
	ReadAll(reader io.Reader) ([]byte, error)
	Stat(name string) (fs.FileInfo, error)
}

type Loader struct {
	fs fsWrapper
}

func NewLoader(fs fsWrapper) *Loader {
	return &Loader{
		fs: fs,
	}
}

func (loader *Loader) LoadDir(root string) ([]*RegoFile, error) {
	result := make([]*RegoFile, 0)
	walker := func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return fmt.Errorf("error accessing path %q: %v", path, err)
		}

		if !info.IsDir() && strings.HasSuffix(info.Name(), ".rego") {
			f, err := loader.LoadRegoFile(path)
			if err != nil {
				return err
			}

			result = append(result, f)
		}

		return nil
	}

	if err := filepath.Walk(root, walker); err != nil {
		return nil, err
	}

	return result, nil
}

func (loader *Loader) LoadRegoFile(path string) (*RegoFile, error) {
	content, err := loader.fs.ReadFile(path)
	if err != nil {
		return nil, fmt.Errorf("error opening file: %v", err)
	}

	parsed, err := ast.ParseModule(path, string(content))
	if err != nil {
		return nil, fmt.Errorf("error parsing file contents: %w", err)
	}

	return &RegoFile{
		Path:   path,
		Parsed: parsed,
	}, nil
}

func (loader *Loader) LoadBundle(path string) (*Bundle, error) {
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

	content, err := loader.extractTarContent(loader.fs.TarReader(gr))
	if err != nil {
		return nil, fmt.Errorf("error extracting tar content: %v", err)
	}

	bundle := &Bundle{
		Name:  filepath.Clean(path),
		Files: make([]*RegoFile, len(content)),
	}

	var i uint
	for k, v := range content {
		parsed, err := ast.ParseModule(k, string(v))
		if err != nil {
			return nil, fmt.Errorf("error parsing file contents: %v", err)
		}

		bundle.Files[i] = &RegoFile{
			Path:   k,
			Parsed: parsed,
		}

		i++
	}

	return bundle, nil
}

func (loader *Loader) extractTarContent(tr *tar.Reader) (map[string][]byte, error) {
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
		if err := validatePath(cleanName); err != nil {
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

func validatePath(path string) error {
	const (
		minPathLength = 1
		maxPathLength = 255
	)

	if len(path) < minPathLength || len(path) > maxPathLength {
		return fmt.Errorf("length is not within the valid")
	}

	if !utf8.ValidString(path) {
		return fmt.Errorf("contains invalid UTF-8 characters")
	}

	invalidPatterns := []string{"..", "://", "\x00"}
	for _, pattern := range invalidPatterns {
		if strings.Contains(path, pattern) {
			return fmt.Errorf("contains invalid pattern '%s'", pattern)
		}
	}

	if strings.Trim(path, " \t\n\r\x00") != path {
		return fmt.Errorf("begins or ends with whitespace or control characters")
	}

	cleanedPath := filepath.Clean(path)

	if cleanedPath != path {
		return fmt.Errorf("potential directory traversal attempt detected")
	}

	if strings.HasPrefix(cleanedPath, "../") {
		return fmt.Errorf("attempts to navigate upwards in the directory hierarchy")
	}

	if filepath.IsAbs(cleanedPath) {
		return fmt.Errorf("absolute paths are not allowed")
	}

	if strings.HasPrefix(cleanedPath, "/") {
		return fmt.Errorf("appears to be an absolute path which is not allowed")
	}

	return nil
}
