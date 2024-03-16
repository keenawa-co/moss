package bundlepkg

import (
	"github.com/4rchr4y/bpm/bpmiface"
	"github.com/4rchr4y/bpm/bundleutil/encode"
	"github.com/4rchr4y/bpm/bundleutil/inspect"
	"github.com/4rchr4y/bpm/bundleutil/manifest"
	"github.com/4rchr4y/bpm/core"
	"github.com/4rchr4y/bpm/fetch"
	"github.com/4rchr4y/bpm/storage"
	"github.com/4rchr4y/bpm/storage/storageiface"
	"github.com/4rchr4y/godevkit/v3/syswrap/ioiface"
	"github.com/4rchr4y/godevkit/v3/syswrap/osiface"
)

type BundlePkgManagerConf struct {
	RootDir  string
	OSWrap   osiface.OSWrapper
	IOWrap   ioiface.IOWrapper
	IOStream core.IO
}

type BundlePkgManager struct {
	encoder    bpmiface.Encoder
	inspector  bpmiface.Inspector
	fetcher    bpmiface.Fetcher
	manifester bpmiface.Manifester
	storage    storageiface.Storage
}

func NewBundlePkgManager(conf BundlePkgManagerConf) *BundlePkgManager {
	bpm := new(BundlePkgManager)

	bpm.encoder = &encode.Encoder{
		IO: conf.IOStream,
	}
	bpm.inspector = &inspect.Inspector{
		IO: conf.IOStream,
	}
	bpm.storage = &storage.Storage{
		Dir:     conf.RootDir,
		IO:      conf.IOStream,
		OSWrap:  conf.OSWrap,
		IOWrap:  conf.IOWrap,
		Encoder: bpm.encoder,
	}

	bpm.fetcher = &fetch.Fetcher{
		IO:        conf.IOStream,
		Storage:   bpm.storage,
		Inspector: bpm.inspector,
		GitHub: &fetch.GithubFetcher{
			IO:      conf.IOStream,
			Client:  nil,
			Encoder: bpm.encoder,
		},
	}

	bpm.manifester = &manifest.Manifester{
		IO:      conf.IOStream,
		OSWrap:  conf.OSWrap,
		Storage: bpm.storage,
		Encoder: bpm.encoder,
		Fetcher: bpm.fetcher,
	}

	return bpm
}

func (bpm *BundlePkgManager) Storage() storageiface.Storage   { return bpm.storage }
func (bpm *BundlePkgManager) Fetcher() bpmiface.Fetcher       { return bpm.fetcher }
func (bpm *BundlePkgManager) Inspector() bpmiface.Inspector   { return bpm.inspector }
func (bpm *BundlePkgManager) Manifester() bpmiface.Manifester { return bpm.manifester }
func (bpm *BundlePkgManager) Encoder() bpmiface.Encoder       { return bpm.encoder }
