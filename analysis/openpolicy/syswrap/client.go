package syswrap

import (
	"os"
	"path/filepath"
)

type Client struct{}

func (Client) ReadFile(name string) ([]byte, error) {
	return os.ReadFile(name)
}

func (Client) Walk(root string, fn filepath.WalkFunc) error {
	return filepath.Walk(root, fn)
}
