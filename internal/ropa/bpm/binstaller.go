package bpm

import (
	"fmt"
	"io"
	"io/fs"
	"os"
	"path/filepath"

	"github.com/4rchr4y/goray/constant"
	"github.com/4rchr4y/goray/internal/ropa/types"
)

type biOSWrapper interface {
	Create(name string) (*os.File, error)
	WriteFile(name string, data []byte, perm fs.FileMode) error
}

type biTOMLEncoder interface {
	Encode(w io.Writer, v interface{}) error
}

type BundleInstaller struct {
	osWrap  biOSWrapper
	encoder biTOMLEncoder
}

type BundleInstallInput struct {
	Dir    string
	Bundle *types.Bundle
}

func (cmd *BundleInstaller) Install(input *BundleInstallInput) error {
	if err := cmd.processRegoFiles(input.Bundle.RegoFiles, input.Dir); err != nil {
		return fmt.Errorf("error occurred rego files processing: %v", err)
	}

	if err := cmd.processBundleLockFile(input.Bundle.BundleLockFile, input.Dir); err != nil {
		return fmt.Errorf("failed to encode %s file: %v", input.Bundle.BundleLockFile.Name(), err)
	}

	if err := cmd.processBundleFile(input.Bundle.BundleFile, input.Dir); err != nil {
		return fmt.Errorf("failed to encode %s file: %v", input.Bundle.BundleFile.Name(), err)
	}

	return nil
}

func (cmd *BundleInstaller) processBundleLockFile(bundleLockFile *types.BundleLockFile, bundleVersionDir string) error {
	file, err := cmd.osWrap.Create(fmt.Sprintf("%s/%s", bundleVersionDir, constant.BPMLockFile))
	if err != nil {
		return err
	}

	if err := cmd.encoder.Encode(file, bundleLockFile); err != nil {
		return err
	}

	return nil
}

func (cmd *BundleInstaller) processBundleFile(bundleFile *types.BundleFile, bundleVersionDir string) error {
	file, err := cmd.osWrap.Create(fmt.Sprintf("%s/%s", bundleVersionDir, constant.BPMFile))
	if err != nil {
		return err
	}

	if err := cmd.encoder.Encode(file, bundleFile); err != nil {
		return err
	}

	return nil
}

func (cmd *BundleInstaller) processRegoFiles(files map[string]*types.RawRegoFile, bundleVersionDir string) error {
	for filePath, file := range files {
		dirPath := filepath.Dir(filePath)
		absPath, err := filepath.Abs(dirPath)
		if err != nil {
			return fmt.Errorf("failed to get absolute path for '%s': %v", dirPath, err)
		}

		if absPath != filepath.Dir(absPath) {
			if err := os.MkdirAll(dirPath, 0755); err != nil {
				return fmt.Errorf("failed to create bundle subfolder '%s': %v", dirPath, err)
			}
		}

		pathToSave := fmt.Sprintf("%s/%s", bundleVersionDir, filePath)
		if err := cmd.osWrap.WriteFile(pathToSave, file.Raw, 0644); err != nil {
			return err
		}
	}

	return nil
}
