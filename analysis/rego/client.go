package rego

import (
	"fmt"
	"io/fs"
	"path/filepath"
)

func ParsePolicyDir(dir string) {
	filepath.WalkDir(dir, func(path string, d fs.DirEntry, err error) error {
		i, err := d.Info()
		if err != nil {
			return err
		}

		fmt.Println(i.Sys())
		return nil
	})
}
