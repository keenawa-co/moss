package plugin

import (
	"context"
	"errors"
	"fmt"
	"log"
	"os"
	"path/filepath"
	"strings"

	"github.com/4rchr4y/go-compass/event"
	"golang.org/x/mod/modfile"
)

type CollectorScanMode int

const (
	// Full Scan Mode: Activated when the configuration file is absent.
	// Unlike standard scanning, this mode not only searches for directories but also locates the `go.mod`
	// file to extract the module value. This ensures a comprehensive discovery of resources and settings.
	ModeScanFull CollectorScanMode = iota
	// Scan Dirs Mode: Unlike the Full Scan Mode, this mode limits its search to directories containing *.go files,
	// identifying them as Go packages. This is more targeted and omits directories that are not relevant to Go development.
	ModeScanDirs
)

const (
	goModFileName = "go.mod"
	goFileExt     = ".go"
)

type CollectorPluginInput struct {
	IgnoredList map[string]struct{}
	PackageDirs []string
	RootDir     string
	TargetDir   string
}

type CollectorPluginOutput struct {
	PackageDirs []string
	ModuleName  string
	LangVersion string
}

type CollectorPlugin struct {
	name    string
	next    Plugin
	eventCh chan event.Event

	input  *CollectorPluginInput
	output *CollectorPluginOutput
}

func (p *CollectorPlugin) Name() string {
	return p.name
}

func (p *CollectorPlugin) IsTerminal() bool {
	return false
}

func (p *CollectorPlugin) Next() Plugin {
	return p.next
}

func (p *CollectorPlugin) SetNext(plugin Plugin) {
	p.next = plugin
}

func (p *CollectorPlugin) Execute(ctx context.Context, input interface{}) interface{} {
	p.input = input.(*CollectorPluginInput)

	if err := p.explore(p.input.RootDir, true); err != nil {
		// p.eventCh <-
		log.Fatal(err)
	}

	return p.input.PackageDirs
}

func (p *CollectorPlugin) explore(path string, isRoot bool) error {
	entries, err := os.ReadDir(path)
	if err != nil {
		return fmt.Errorf("failed to read directory: %w", err)
	}

	subdirs, goFilesExist, err := p.scanDir(entries, path)
	if err != nil {
		return err
	}

	if goFilesExist && strings.HasPrefix(path, p.input.TargetDir) {
		p.input.PackageDirs = append(p.input.PackageDirs, path)
	}

	if err := p.exploreSubDir(subdirs); err != nil {
		return err
	}

	if !isRoot {
		return nil
	}

	if len(p.output.ModuleName) < 1 || len(p.output.LangVersion) < 1 {
		return errors.New("couldn't find the go.mod file")
	}

	return nil
}

func (c *CollectorPlugin) scanDir(entries []os.DirEntry, root string) ([]string, bool, error) {
	var subdirs []string
	goFilesExist := false

	for _, entry := range entries {
		entryName := entry.Name()

		if entry.IsDir() {
			if _, exists := c.input.IgnoredList[entryName]; exists {
				continue
			}
			subdirs = append(subdirs, filepath.Join(root, entryName))
			continue
		}

		if entryName == goModFileName {
			if err := c.processGoMod(root); err != nil {
				return nil, false, fmt.Errorf("failed to process go.mod: %w", err)
			}
			continue
		}

		if filepath.Ext(entryName) == goFileExt {
			goFilesExist = true
		}
	}

	return subdirs, goFilesExist, nil
}

func (c *CollectorPlugin) exploreSubDir(subdirs []string) error {
	for _, subdir := range subdirs {
		if err := c.explore(subdir, false); err != nil {
			return err
		}
	}

	return nil
}

func (c *CollectorPlugin) processGoMod(root string) error {
	goModPath := filepath.Join(root, goModFileName)
	content, err := os.ReadFile(goModPath)
	if err != nil {
		return err
	}

	modFileData, err := modfile.Parse(goModFileName, content, nil)
	if err != nil {
		return err
	}

	c.output.ModuleName = modFileData.Module.Mod.Path
	c.output.LangVersion = modFileData.Go.Version

	return nil
}
