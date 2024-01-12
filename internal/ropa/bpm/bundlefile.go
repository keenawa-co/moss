package bpm

import (
	"fmt"
	"io"
)

type BundleFile struct {
	Package *PackageDef `toml:"package" validate:"required"`
}

type PackageDef struct {
	Name        string   `toml:"name" validate:"required"`
	Version     float64  `toml:"version" validate:"required"`
	Author      []string `toml:"author"`
	Description string   `toml:"description"`
}

func (bf *BundleFile) Validate(validator validateClient) error {
	if err := validator.ValidateStruct(bf); err != nil {
		return fmt.Errorf("failed to validate bundle.toml file: %v", err)
	}

	return nil
}

func DecodeBundleFile(iowrap ioWrapper, toml tomlClient, reader io.Reader) (*BundleFile, error) {
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
