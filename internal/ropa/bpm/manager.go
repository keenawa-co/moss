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

type validateClient interface {
	ValidateStruct(s interface{}) error
}

type Bpm struct {
	toml     tomlClient
	tar      tarClient
	iowrap   ioWrapper
	validate validateClient
}

type BpmConf struct {
}

func NewBpm(ts tomlClient, tar tarClient, io ioWrapper, v validateClient) *Bpm {
	return &Bpm{
		toml:     ts,
		tar:      tar,
		iowrap:   io,
		validate: v,
	}
}

func (bpm *Bpm) Pack(sourcePath string, destPath string, bundleName string) error {
	if err := bpm.tar.Compress(sourcePath, destPath, bundleName); err != nil {
		return fmt.Errorf("error occurred while packing '%s' bundle: %v", bundleName, err)
	}

	return nil
}
