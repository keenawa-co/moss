package goray

import (
	"encoding/json"
	"fmt"
	"sync"

	"github.com/4rchr4y/goray/obj"
	"golang.org/x/mod/modfile"
)

type exploreClient interface {
	Explore() (*modfile.File, []string, error)
}

type Compass struct {
	noCopy    noCopy
	noCompare noCompare

	MaxParserConcurrency uint

	engine *Engine
	parser exploreClient
}

func (c *Compass) Scan() error {
	modfile, dirGroup, err := c.parser.Explore()
	if err != nil {
		return err
	}

	var wg sync.WaitGroup
	sema := make(chan struct{}, c.MaxParserConcurrency)
	c.engine.modfile = modfile

	for _, dir := range dirGroup {
		wg.Add(1)

		go func(dir string) {
			defer wg.Done()
			sema <- struct{}{}
			defer func() { <-sema }()

			data, err := c.engine.ParseDir(dir)
			if err != nil {
				fmt.Println(err)
			}

			debugPkg(data)
		}(dir)
	}

	wg.Wait()
	return nil
}

func New(cfg *Config) *Compass {
	return &Compass{
		MaxParserConcurrency: 1,
		engine: &Engine{
			MaxEngineConcurrency: 10,
			group:                cfg.Group,
		},
		parser: &explorer{
			RootDir:     cfg.RootDir,
			TargetDir:   cfg.TargetDir,
			IgnoredList: cfg.IgnoredList,
		},
	}
}

func debugPkg(data []*obj.PackageObj) {
	for _, pkg := range data {
		jsonData, err := json.Marshal(pkg)
		if err != nil {
			fmt.Println(err)
		}

		fmt.Println("\n", string(jsonData))
	}

}
