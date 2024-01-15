package loader

import (
	"fmt"
	"path/filepath"

	"github.com/4rchr4y/goray/constant"
	"github.com/4rchr4y/goray/internal/ropa/types"
	"github.com/open-policy-agent/opa/ast"
)

type tomlDecoder interface {
	Decode(data string, v interface{}) error
}

type BundleProcessor struct {
	toml tomlDecoder
}

type ProcessInput struct {
	BundlePath string
	Files      map[string][]byte
}

func (bp *BundleProcessor) Process(input *ProcessInput) (*types.Bundle, error) {
	bundle := &types.Bundle{
		FileName:  filepath.Clean(input.BundlePath),
		RegoFiles: make(map[string]*types.RawRegoFile),
	}

	for filePath, content := range input.Files {
		switch {
		case isRegoFile(filePath):
			parsed, err := bp.processRegoFile(content, filePath)
			if err != nil {
				return nil, err
			}

			bundle.RegoFiles[filePath] = &types.RawRegoFile{
				Path:   filePath,
				Parsed: parsed,
			}

		case isBPMFile(filePath):
			bundlefile, err := bp.processBPMFile(content)
			if err != nil {
				return nil, err
			}

			bundle.BundleFile = bundlefile

		case isBPMLockFile(filePath):
			bundlelock, err := bp.processBPMLockFile(content)
			if err != nil {
				return nil, err
			}

			bundle.BundleLockFile = bundlelock

		case isBPMWorkFile(filePath):
			bpmwork, err := bp.processBPMWorkFile(content)
			if err != nil {
				return nil, err
			}

			bundle.BpmWorkFile = bpmwork
		}
	}

	return bundle, nil
}

func (bp *BundleProcessor) processRegoFile(fileContent []byte, filePath string) (*ast.Module, error) {
	parsed, err := ast.ParseModule(filePath, string(fileContent))
	if err != nil {
		return nil, fmt.Errorf("error parsing file contents: %v", err)
	}

	return parsed, nil
}

func (bp *BundleProcessor) processBPMWorkFile(fileContent []byte) (*types.BpmWorkFile, error) {
	var bpmwork types.BpmWorkFile
	if err := bp.toml.Decode(string(fileContent), &bpmwork); err != nil {
		return nil, fmt.Errorf("error parsing bpm.work content: %v", err)
	}

	return &bpmwork, nil
}

func (bp *BundleProcessor) processBPMLockFile(fileContent []byte) (*types.BundleLockFile, error) {
	var bundlelock types.BundleLockFile
	if err := bp.toml.Decode(string(fileContent), &bundlelock); err != nil {
		return nil, fmt.Errorf("error parsing bundle.lock content: %v", err)
	}

	return &bundlelock, nil
}

func (bp *BundleProcessor) processBPMFile(fileContent []byte) (*types.BundleFile, error) {
	var bundlefile types.BundleFile
	if err := bp.toml.Decode(string(fileContent), &bundlefile); err != nil {
		return nil, fmt.Errorf("error parsing bundle.toml content: %v", err)
	}

	return &bundlefile, nil
}

func isRegoFile(filePath string) bool    { return filepath.Ext(filePath) == constant.RegoExt }
func isBPMFile(filePath string) bool     { return filePath == constant.BPMFile }
func isBPMLockFile(filePath string) bool { return filePath == constant.BPMLockFile }
func isBPMWorkFile(filePath string) bool { return filePath == constant.BPMWorkFile }
