package bpm

import (
	"fmt"
	"io"
)

type BundleLock struct {
	Version int          `toml:"version"`
	Modules []*ModuleDef `toml:"modules"`
}

type ModuleDef struct {
	Name         string   `toml:"name"`
	Source       string   `toml:"source"`
	Checksum     string   `toml:"checksum"`
	Dependencies []string `toml:"dependencies"`
}

func EncodeBundleLock(ts tomlClient, w io.Writer, bl *BundleLock) error {
	if err := ts.Encode(w, bl); err != nil {
		return fmt.Errorf("failed to encode bundle.lock file: %v", err)
	}

	return nil
}
