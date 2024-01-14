package syswrap

import (
	"archive/tar"
	"compress/gzip"
	"io"
	"io/fs"
	"os"
	"path/filepath"
)

type OsWrapper struct{}

func (OsWrapper) Create(name string) (*os.File, error) {
	return os.Create(name)
}

func (OsWrapper) Open(name string) (*os.File, error) {
	return os.Open(name)
}

func (OsWrapper) LookupEnv(key string) (string, bool) {
	return os.LookupEnv(key)
}

func (OsWrapper) UserHomeDir() (string, error) {
	return os.UserHomeDir()
}

func (OsWrapper) Mkdir(name string, perm fs.FileMode) error {
	return os.Mkdir(name, perm)
}

func (OsWrapper) Stat(name string) (fs.FileInfo, error) {
	return os.Stat(name)
}

func (OsWrapper) ReadFile(name string) ([]byte, error) {
	return os.ReadFile(name)
}

func (OsWrapper) GzipReader(reader io.Reader) (*gzip.Reader, error) {
	return gzip.NewReader(reader)
}

func (OsWrapper) GzipWriter(writer io.Writer) *gzip.Writer {
	return gzip.NewWriter(writer)
}

func (OsWrapper) TarReader(reader io.Reader) *tar.Reader {
	return tar.NewReader(reader)
}

func (OsWrapper) TarWriter(writer io.Writer) *tar.Writer {
	return tar.NewWriter(writer)
}

func (OsWrapper) Walk(root string, fn filepath.WalkFunc) error {
	return filepath.Walk(root, fn)
}
