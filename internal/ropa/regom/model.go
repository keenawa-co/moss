package regom

import "github.com/open-policy-agent/opa/ast"

type (
	RegoFile struct {
		Path   string
		Raw    []byte
		Parsed *ast.Module
	}

	RegoFileMeta struct {
		Dependencies []Path
	}
)

type Bundle struct {
	Name  string
	Files []*RegoFile
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
		File    *RegoFile
		Targets []Path
		Deps    []Path
	}

	Policy struct {
		File    *RegoFile
		Targets TargetGroup
		Deps    []*RegoFile
	}
)
