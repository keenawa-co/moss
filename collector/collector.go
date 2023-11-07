package collector

import (
	"errors"
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"golang.org/x/mod/modfile"
)

const goModFileName = "go.mod"
const goFileExt = ".go"

type Collector struct {
	packageDirs []string
	info        *Info
	rootDir     string
	targetDir   string
	ignoredList map[string]struct{}
}

func (c *Collector) GetAllPackageDirs() []string {
	return c.packageDirs
}

func (c *Collector) GetInfo() *Info {
	return c.info
}

func (c *Collector) Explore() error {
	return c.explore(c.rootDir, true)
}

func (c *Collector) explore(path string, isRoot bool) error {
	entries, err := os.ReadDir(path)
	if err != nil {
		return fmt.Errorf("failed to read directory: %w", err)
	}

	subdirs, goFilesExist, err := c.scanDir(entries, path)
	if err != nil {
		return err
	}

	if goFilesExist && strings.HasPrefix(path, c.targetDir) {
		c.packageDirs = append(c.packageDirs, path)
	}

	if err := c.exploreSubDir(subdirs); err != nil {
		return err
	}

	if !isRoot {
		return nil
	}

	if len(c.info.ModuleName) < 1 || len(c.info.LangVersion) < 1 {
		return errors.New("couldn't find the go.mod file")
	}

	return nil
}

func (c *Collector) scanDir(entries []os.DirEntry, root string) ([]string, bool, error) {
	var subdirs []string
	goFilesExist := false

	for _, entry := range entries {
		entryName := entry.Name()

		if entry.IsDir() {
			if _, exists := c.ignoredList[entryName]; exists {
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

func (c *Collector) exploreSubDir(subdirs []string) error {
	for _, subdir := range subdirs {
		if err := c.explore(subdir, false); err != nil {
			return err
		}
	}

	return nil
}

func (c *Collector) processGoMod(root string) error {
	goModPath := filepath.Join(root, goModFileName)
	content, err := os.ReadFile(goModPath)
	if err != nil {
		return err
	}

	modFileData, err := modfile.Parse(goModFileName, content, nil)
	if err != nil {
		return err
	}

	c.info.ModuleName = modFileData.Module.Mod.Path
	c.info.LangVersion = modFileData.Go.Version

	return nil
}

type NewCollectorParam struct {
	RootDir     string
	TargetDir   string
	IgnoredList map[string]struct{}
}

func NewCollector(param *NewCollectorParam) *Collector {
	return &Collector{
		ignoredList: param.IgnoredList,
		rootDir:     param.RootDir,
		targetDir:   param.TargetDir,
		info:        &Info{},
		packageDirs: make([]string, 0),
	}
}
