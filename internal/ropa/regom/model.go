package regom

import "github.com/open-policy-agent/opa/ast"

type RawRegoFile struct {
	Path   string
	Parsed *ast.Module
}

type (
	RegoFileMeta struct {
		Dependencies []Path
	}
)

type Bundle struct {
	Name  string
	Files []*RawRegoFile
}

type DepType int

const (
	Invalid DepType = iota
	File
	Dir
)

type Path struct {
	Path string
	Type DepType
}

func (p Path) IsDir() bool {
	return p.Type == Dir
}

type TargetGroup struct {
	Hash string // hash, based on a list of targets
	List []Path // list of targets
}

type (
	PolicySpec struct {
		File    *RawRegoFile
		Targets []Path
		Deps    []Path
	}

	Policy struct {
		File    *RawRegoFile
		Targets TargetGroup
		Deps    []*RawRegoFile
	}
)
