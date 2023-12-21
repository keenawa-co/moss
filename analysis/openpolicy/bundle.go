package openpolicy

import (
	"archive/tar"
	"compress/gzip"
	"fmt"
	"io"
	"os"

	"github.com/open-policy-agent/opa/ast"
)

type ModuleFile struct {
	Path   string
	Raw    []byte
	Parsed *ast.Module
}

type Bundle struct {
	Modules []*ModuleFile
}

func LoadBundle(path string) (*Bundle, error) {
	file, err := os.Open(path)
	if err != nil {
		return nil, fmt.Errorf("error opening file: %w", err)
	}
	defer file.Close()

	gzr, err := gzip.NewReader(file)
	if err != nil {
		return nil, fmt.Errorf("error creating gzip reader: %w", err)
	}
	defer gzr.Close()

	tr := tar.NewReader(gzr)
	fileContents := make(map[string][]byte)

	for {
		header, err := tr.Next()
		if err == io.EOF {
			break
		}
		if err != nil {
			return nil, fmt.Errorf("error reading tar entry: %w", err)
		}

		if header.Typeflag == tar.TypeReg {
			buf, err := io.ReadAll(tr)
			if err != nil {
				return nil, fmt.Errorf("error reading file contents: %w", err)
			}
			fileContents[header.Name] = buf
		}
	}

	for k, v := range fileContents {
		fmt.Println(k)
		fmt.Println(string(v))
	}
	return nil, nil
}
