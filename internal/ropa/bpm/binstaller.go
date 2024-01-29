package bpm

// --- Temporary documentation ---
// So, the main task of this 'service' it to install the bundle.
// Installer should not care about the place where this should be installed,
// also he is not a right place to resolve a conflicts with other versions e.g.
// He just parse the passed bundle, validate correctness of all files.
// Basically this is the last stage of installing process.

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
		pathToSave := filepath.Join(bundleVersionDir, filePath)
		dirToSave := filepath.Dir(pathToSave)

		if _, err := os.Stat(dirToSave); os.IsNotExist(err) {
			if err := os.MkdirAll(dirToSave, 0755); err != nil {
				return fmt.Errorf("failed to create directory '%s': %v", dirToSave, err)
			}
		} else if err != nil {
			return fmt.Errorf("error checking directory '%s': %v", dirToSave, err)
		}

		if err := cmd.osWrap.WriteFile(pathToSave, file.Raw, 0644); err != nil {
			return fmt.Errorf("failed to write file '%s': %v", pathToSave, err)
		}
	}

	return nil
}
