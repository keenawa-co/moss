package openpolicy

import (
	"os"
	"path/filepath"
	"strings"

	"github.com/open-policy-agent/opa/ast"
)

const (
	regoFileExt    = ".rego"
	localImportSuf = "data."
)

func IsStdImport(std map[string]string, moduleImport *ast.Import) (string, bool) {
	moduleImportStr := moduleImport.Path.String()
	if !strings.HasPrefix(moduleImport.Path.String(), localImportSuf) {
		return "", false
	}

	importPath, ok := std[moduleImportStr[len(localImportSuf):]]
	return importPath, ok
}

func DefineStdLib(root string) (map[string]string, error) {
	std := make(map[string]string)
	err := filepath.Walk(root, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return err
		}

		if !info.IsDir() && strings.HasSuffix(path, regoFileExt) {
			formattedPath := formatPath(trimPath(root, path))
			std[formattedPath] = path
		}

		return nil
	})

	return std, err
}

func trimPath(stdDirPath string, libPath string) string {
	return strings.Clone(libPath[len(stdDirPath)+1 : len(libPath)-len(regoFileExt)])
}

func formatPath(path string) string {
	return strings.ReplaceAll(path, "/", ".")
}
