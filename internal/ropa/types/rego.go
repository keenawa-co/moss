package types

import "github.com/open-policy-agent/opa/ast"

type RawRegoFile struct {
	Path   string
	Parsed *ast.Module
}

type Bundle struct {
	Name           string
	BundleFile     *BundleFile
	BundleLockFile *BundleLockFile
	BpmWorkFile    *BpmWorkFile
	RegoFiles      map[string]*RawRegoFile
}
