package component

import "github.com/4rchr4y/goray/internal/schematica"

type Schema struct {
	Version int64
	Root    *schematica.Block
}
