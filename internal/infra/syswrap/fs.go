package syswrap

import (
	"archive/tar"
	"compress/gzip"
	"io"
	"os"
	"path/filepath"
)

type FsWrapper struct{}

// TODO: move to os.go file
func (FsWrapper) OpenFile(name string) (*os.File, error) {
	return os.Open(name)
}

func (FsWrapper) GzipReader(reader io.Reader) (*gzip.Reader, error) {
	return gzip.NewReader(reader)
}

func (FsWrapper) GzipWriter(writer io.Writer) *gzip.Writer {
	return gzip.NewWriter(writer)
}

func (FsWrapper) TarReader(reader io.Reader) *tar.Reader {
	return tar.NewReader(reader)
}

func (FsWrapper) TarWriter(writer io.Writer) *tar.Writer {
	return tar.NewWriter(writer)
}

// TODO: move to os.go file
func (FsWrapper) ReadFile(name string) ([]byte, error) {
	return os.ReadFile(name)
}

// TODO: remove this func
func (FsWrapper) ReadAll(reader io.Reader) ([]byte, error) {
	return io.ReadAll(reader)
}

func (FsWrapper) Walk(root string, fn filepath.WalkFunc) error {
	return filepath.Walk(root, fn)
}
