package syswrap

import (
	"io/fs"
	"os"
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
