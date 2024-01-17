package types

import (
	"fmt"
	"regexp"

	"github.com/4rchr4y/goray/constant"
)

type BPMFile interface {
	Name() string

	bpmFile()
}

// ----------------- Bundle File ----------------- //

type PackageDef struct {
	Name        string   `toml:"name" validate:"required"`
	Version     string   `toml:"version" validate:"required"`
	Author      []string `toml:"author"`
	Description string   `toml:"description"`
}

type BundleFile struct {
	Package *PackageDef `toml:"package" validate:"required"`
}

func (*BundleFile) bpmFile()     {}
func (*BundleFile) Name() string { return constant.BPMFile }

type validateClient interface {
	ValidateStruct(s interface{}) error
}

var versionRegex = regexp.MustCompile(`^\d+\.\d+\.\d+$`)

func (bf *BundleFile) Validate(validator validateClient) error {
	if ok := versionRegex.MatchString(bf.Package.Version); !ok {
		return fmt.Errorf("failed to validate '%s' file, expected version X.Y.Z (Major.Minor.Patch) format ", bf.Package.Version)
	}

	if err := validator.ValidateStruct(bf); err != nil {
		return fmt.Errorf("failed to validate %s file: %v", bf.Name(), err)
	}

	return nil
}

// ----------------- Bundle Lock File ----------------- //

type ModuleDef struct {
	Name         string   `toml:"name"`
	Source       string   `toml:"source"`
	Checksum     string   `toml:"checksum"`
	Dependencies []string `toml:"dependencies"`
}

type BundleLockFile struct {
	Version int          `toml:"version"`
	Modules []*ModuleDef `toml:"modules"`
}

func (*BundleLockFile) bpmFile()     {}
func (*BundleLockFile) Name() string { return constant.BPMLockFile }

// ----------------- Bundle Work File ----------------- //

type WorkspaceDef struct {
	Path     string   `toml:"path"`
	Author   []string `toml:"author"`
	Packages []string `toml:"packages"`
}

type BpmWorkFile struct {
	Workspace *WorkspaceDef `toml:"workspace"`
}

func (*BpmWorkFile) bpmFile()     {}
func (*BpmWorkFile) Name() string { return constant.BPMWorkFile }
