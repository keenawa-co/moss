package syswrap

import "os"

type OsClient struct{}

func (OsClient) LookupEnv(key string) (string, bool) {
	return os.LookupEnv(key)
}
