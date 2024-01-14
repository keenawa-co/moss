package types

import (
	"fmt"
	"io"
)

// ----------------- Bundle File ----------------- //

type BundleFile struct {
	Package *PackageDef `toml:"package" validate:"required"`
}

type PackageDef struct {
	Name        string   `toml:"name" validate:"required"`
	Version     float64  `toml:"version" validate:"required"`
	Author      []string `toml:"author"`
	Description string   `toml:"description"`
}

type validateClient interface {
	ValidateStruct(s interface{}) error
}

func (bf *BundleFile) Validate(validator validateClient) error {
	if err := validator.ValidateStruct(bf); err != nil {
		return fmt.Errorf("failed to validate bundle.toml file: %v", err)
	}

	return nil
}

type ioWrapper interface {
	ReadAll(r io.Reader) ([]byte, error)
}

type tomlDecoder interface {
	Decode(data string, v interface{}) error
}

func DecodeBundleFile(iowrap ioWrapper, toml tomlDecoder, reader io.Reader) (*BundleFile, error) {
	content, err := iowrap.ReadAll(reader)
	if err != nil {
		return nil, err
	}

	var bundlefile BundleFile
	if err := toml.Decode(string(content), &bundlefile); err != nil {
		return nil, err
	}

	return &bundlefile, nil
}

// ----------------- Bundle Lock File ----------------- //

type BundleLockFile struct {
	Version int          `toml:"version"`
	Modules []*ModuleDef `toml:"modules"`
}

type ModuleDef struct {
	Name         string   `toml:"name"`
	Source       string   `toml:"source"`
	Checksum     string   `toml:"checksum"`
	Dependencies []string `toml:"dependencies"`
}

type tomlEncoder interface {
	Encode(w io.Writer, v interface{}) error
}

func EncodeBundleLock(ts tomlEncoder, w io.Writer, bl *BundleLockFile) error {
	if err := ts.Encode(w, bl); err != nil {
		return fmt.Errorf("failed to encode bundle.lock file: %v", err)
	}

	return nil
}

// ----------------- Bundle Work File ----------------- //

type BpmWorkFile struct {
	Workspace *WorkspaceDef `toml:"workspace"`
}

type WorkspaceDef struct {
	Path     string   `toml:"path"`
	Author   []string `toml:"author"`
	Packages []string `toml:"packages"`
}
