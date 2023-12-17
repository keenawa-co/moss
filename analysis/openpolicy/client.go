package openpolicy

import (
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"github.com/4rchr4y/goray/analysis/openpolicy/syswrap"
)

const (
	regoFileExt    = ".rego"
	localImportSuf = "data."
)

type sysWrapper interface {
	ReadFile(name string) ([]byte, error)
	Walk(root string, fn filepath.WalkFunc) error
}

type LazyLoaderFn = func() (*Module, error)

type RegoClient struct {
	cache  map[string]*Module      // paths to already processed files
	lazy   map[string]LazyLoaderFn // rego import -> loader func
	system sysWrapper              // OS client
}

type regoCliOptFn func(*RegoClient)

func NewRegoClient(pathToStd string, options ...regoCliOptFn) (client *RegoClient, err error) {
	client = &RegoClient{
		cache:  make(map[string]*Module),
		system: new(syswrap.Client),
	}

	client.lazy, err = DefineStandardLib(client, pathToStd)
	if err != nil {
		return nil, fmt.Errorf("failed to preload std: %v", err)
	}

	return client, nil
}

func DefineStandardLib(regoCli *RegoClient, root string) (map[string]LazyLoaderFn, error) {
	lazy := make(map[string]LazyLoaderFn)
	err := regoCli.system.Walk(root, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return err
		}

		if !info.IsDir() && strings.HasSuffix(path, regoFileExt) {
			formattedPath := formatPath(trimPath(root, path))
			loadFn := func() (*Module, error) { return LoadModule(regoCli, path) }

			lazy[formattedPath] = loadFn
		}

		return nil
	})

	return lazy, err
}

func trimPath(stdDirPath string, libPath string) string {
	return strings.Clone(libPath[len(stdDirPath)+1 : len(libPath)-len(regoFileExt)])
}

func formatPath(path string) string {
	return strings.ReplaceAll(path, "/", ".")
}
