package openpolicy

import (
	"strings"

	"github.com/4rchr4y/goray/pkg/radix"
)

type registry struct {
	idxByPath    *radix.Tree[int]
	idxByPackage *radix.Tree[int]
	store        []*RegoFile
}

func (r *registry) insert(file *RegoFile) {
	r.idxByPath.Store([]byte(file.Path), len(r.store))
	r.idxByPackage.Store([]byte(file.Parsed.Package.Path.String()), len(r.store))
	r.store = append(r.store, file)
}

func (r *registry) load(key string) ([]*RegoFile, bool) {
	if strings.Contains(key, "/") {
		idxList, exists := r.idxByPath.LoadPrefix([]byte(key))
		if !exists {
			return nil, false
		}

		result := make([]*RegoFile, len(idxList))
		for idx := range idxList {
			result[idx] = r.store[idxList[idx].Value]
		}

		return result, true
	}

	if strings.Contains(key, ".") {
		idxList, exists := r.idxByPackage.LoadPrefix([]byte(key))
		if !exists {
			return nil, false
		}
		result := make([]*RegoFile, len(idxList))
		for idx := range idxList {
			result[idx] = r.store[idxList[idx].Value]
		}

		return result, true
	}

	return nil, true
}

func newRegistry() *registry {
	return &registry{
		idxByPath:    radix.NewTree[int](),
		idxByPackage: radix.NewTree[int](),
		store:        make([]*RegoFile, 0),
	}
}
