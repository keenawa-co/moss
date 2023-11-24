package goray

import (
	"errors"
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"golang.org/x/mod/modfile"
)

const (
	goModFileName = "go.mod"
	goFileExt     = ".go"
)

type explorer struct {
	RootDir     string
	TargetDir   string
	IgnoredList map[string]struct{}
	PackageDirs []string
	Modfile     *modfile.File
}

func (c *explorer) Explore() (*modfile.File, []string, error) {
	if err := c.explore(c.RootDir, true); err != nil {
		return nil, nil, err
	}

	return c.Modfile, c.PackageDirs, nil
}

func (c *explorer) explore(path string, isRoot bool) error {
	entries, err := os.ReadDir(path)
	if err != nil {
		return fmt.Errorf("failed to read directory: %w", err)
	}

	subdirs, goFilesExist, err := c.exploreDir(entries, path)
	if err != nil {
		return err
	}

	if goFilesExist && strings.HasPrefix(path, c.TargetDir) {
		c.PackageDirs = append(c.PackageDirs, path)
	}

	if err := c.exploreSubDir(subdirs); err != nil {
		return err
	}

	if !isRoot {
		return nil
	}

	if c.Modfile == nil {
		return errors.New("couldn't find the go.mod file")
	}

	return nil
}

func (c *explorer) exploreDir(entries []os.DirEntry, root string) ([]string, bool, error) {
	var subdirs []string
	goFilesExist := false

	for _, entry := range entries {
		entryName := entry.Name()

		if entry.IsDir() {
			if _, exists := c.IgnoredList[entryName]; exists {
				continue
			}
			subdirs = append(subdirs, filepath.Join(root, entryName))
			continue
		}

		if entryName == goModFileName {
			modfile, err := c.processGoMod(root)
			if err != nil {
				return nil, false, fmt.Errorf("failed to process go.mod: %w", err)
			}

			c.Modfile = modfile
			continue
		}

		if filepath.Ext(entryName) == goFileExt {
			goFilesExist = true
		}
	}

	return subdirs, goFilesExist, nil
}

func (c *explorer) exploreSubDir(subdirs []string) error {
	for _, subdir := range subdirs {
		if err := c.explore(subdir, false); err != nil {
			return err
		}
	}

	return nil
}

func (c *explorer) processGoMod(root string) (*modfile.File, error) {
	path := filepath.Join(root, goModFileName)
	content, err := os.ReadFile(path)
	if err != nil {
		return nil, err
	}

	return modfile.Parse(goModFileName, content, nil)
}
