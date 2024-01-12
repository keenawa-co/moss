package bpm

import (
	"fmt"
	"io"
)

type tomlClient interface {
	Decode(data string, v interface{}) error
}

type tarClient interface {
	Compress(dirPath string, targetDir string, archiveName string) error
}

type ioWrapper interface {
	ReadAll(r io.Reader) ([]byte, error)
}

type Bpm struct {
	toml   tomlClient
	tar    tarClient
	iowrap ioWrapper
}

type BpmConf struct {
}

func NewBpm(ts tomlClient, tar tarClient, io ioWrapper) *Bpm {
	return &Bpm{
		toml:   ts,
		tar:    tar,
		iowrap: io,
	}
}

func (bpm *Bpm) Pack(sourcePath string, destPath string, bundleName string) error {
	if err := bpm.tar.Compress(sourcePath, destPath, bundleName); err != nil {
		return fmt.Errorf("error occurred while packing '%s' bundle: %v", bundleName, err)
	}

	return nil
}

func (bpm *Bpm) ParseBundleFile(reader io.Reader) (*BundleFile, error) {
	content, err := bpm.iowrap.ReadAll(reader)
	if err != nil {
		return nil, err
	}

	var bundlefile BundleFile
	if err := bpm.toml.Decode(string(content), &bundlefile); err != nil {
		return nil, err
	}

	return &bundlefile, nil
}
