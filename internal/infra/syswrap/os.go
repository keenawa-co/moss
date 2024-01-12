package syswrap

import "os"

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
