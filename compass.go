package compass

import (
	"encoding/json"
	"fmt"
	"sync"

	"github.com/4rchr4y/go-compass/core"
	"github.com/4rchr4y/go-compass/obj"
	"github.com/4rchr4y/go-compass/service"
	"golang.org/x/mod/modfile"
)

type parserClient interface {
	Explore() (*modfile.File, []string, error)
}

type Compass struct {
	noCopy    core.NoCopy
	noCompare core.NoCompare

	MaxDirParserCount uint

	engine *Engine
	parser parserClient
}

func (c *Compass) Scan() error {
	modfile, dirGroup, err := c.parser.Explore()
	if err != nil {
		return err
	}

	var wg sync.WaitGroup
	sema := make(chan struct{}, c.MaxDirParserCount)
	c.engine.modfile = modfile

	for _, dir := range dirGroup {
		wg.Add(1)
		sema <- struct{}{}

		go func(dir string) {
			defer wg.Done()
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
		MaxDirParserCount: 1,
		engine: &Engine{
			MaxFileParserCount: 10,
			group:              cfg.Group,
		},
		parser: &service.Parser{
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
