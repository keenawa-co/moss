package syswrap

import (
	"archive/tar"
	"compress/gzip"
	"io"
	"io/fs"
	"os"
	"path/filepath"
)

// TODO: FsWrapper
type FsClient struct{}

func (FsClient) OpenFile(name string) (*os.File, error) {
	return os.Open(name)
}

func (FsClient) GzipReader(reader io.Reader) (*gzip.Reader, error) {
	return gzip.NewReader(reader)
}

func (FsClient) GzipWriter(writer io.Writer) *gzip.Writer {
	return gzip.NewWriter(writer)
}

func (FsClient) TarReader(reader io.Reader) *tar.Reader {
	return tar.NewReader(reader)
}

func (FsClient) TarWriter(writer io.Writer) *tar.Writer {
	return tar.NewWriter(writer)
}

func (FsClient) ReadFile(name string) ([]byte, error) {
	return os.ReadFile(name)
}

func (FsClient) ReadAll(reader io.Reader) ([]byte, error) {
	return io.ReadAll(reader)
}

func (FsClient) Walk(root string, fn filepath.WalkFunc) error {
	return filepath.Walk(root, fn)
}

func (FsClient) Stat(name string) (fs.FileInfo, error) {
	return os.Stat(name)
}
