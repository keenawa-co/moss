package types

import (
	"crypto/md5"
	"encoding/hex"

	"github.com/open-policy-agent/opa/ast"
)

type RawRegoFile struct {
	Path   string
	Raw    []byte
	Parsed *ast.Module
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

func (b *Bundle) Validation() error {
	panic("not implemented")
}
