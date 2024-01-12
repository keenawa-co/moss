package syswrap

import "os"

// TODO: OsWrapper
type OsClient struct{}

func (OsClient) Create(name string) (*os.File, error) {
	return os.Create(name)
}

func (OsClient) Open(name string) (*os.File, error) {
	return os.Open(name)
}

func (OsClient) LookupEnv(key string) (string, bool) {
	return os.LookupEnv(key)
}
