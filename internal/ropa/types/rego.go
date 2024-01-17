package types

import (
	"crypto/md5"
	"encoding/hex"

	"github.com/4rchr4y/goray/version"
	"github.com/open-policy-agent/opa/ast"
)

type RawRegoFile struct {
	Path   string
	Raw    []byte
	Parsed *ast.Module
}

func (rrf *RawRegoFile) Package() string {
	return rrf.Parsed.Package.Path.String()
}

func (rrf *RawRegoFile) Sum() string {
	hash := md5.Sum([]byte(rrf.Parsed.String()))
	return hex.EncodeToString(hash[:])
}

type Bundle struct {
	FileName       string
	BundleFile     *BundleFile
	BundleLockFile *BundleLockFile
	BpmWorkFile    *BpmWorkFile
	RegoFiles      map[string]*RawRegoFile
}

func (b *Bundle) UpdateLock() {
	if len(b.RegoFiles) < 1 {
		return
	}

	if b.BundleLockFile == nil {
		b.BundleLockFile = &BundleLockFile{
			Version: version.BPM,
			Modules: make([]*ModuleDef, len(b.RegoFiles)),
		}
	}

	var i uint
	for path, file := range b.RegoFiles {
		b.BundleLockFile.Modules[i] = &ModuleDef{
			Name:     file.Package(),
			Source:   path,
			Checksum: file.Sum(),
			Dependencies: func() []string {
				result := make([]string, len(file.Parsed.Imports))
				for i, _import := range file.Parsed.Imports {
					result[i] = _import.Path.String()
				}

				return result
			}(),
		}

		i++
	}
}

func (b *Bundle) Validation() error {
	panic("not implemented")
}
