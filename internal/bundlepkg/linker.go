package bundlepkg

import (
	"github.com/4rchr4y/bpm/bpmiface"
	"github.com/4rchr4y/bpm/pkg/linker"
)

type LinkerConf struct {
	Fetcher    bpmiface.Fetcher
	Inspector  bpmiface.Inspector
	Manifester bpmiface.Manifester
}

func NewBundlePkgLinker(conf LinkerConf) *linker.Linker {
	return &linker.Linker{
		Fetcher:    conf.Fetcher,
		Manifester: conf.Manifester,
		Inspector:  conf.Inspector,
	}
}
