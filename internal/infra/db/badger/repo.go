package badger

import (
	"encoding/json"
	"fmt"

	"github.com/4rchr4y/goray/internal/ropa"
)

type LinkerRepo struct {
	projPrefix string
	db         badgerClient
}

func (repo *LinkerRepo) buildKey(prefix string) []byte {
	return []byte(fmt.Sprintf("%s:policy:%s", repo.projPrefix, prefix))
}

func (repo *LinkerRepo) Store(file *ropa.IndexedRegoFile) error {
	marshaled, err := json.Marshal(file)
	if err != nil {
		return err
	}

	return repo.db.Set(repo.buildKey(file.Path), marshaled)
}

func (repo *LinkerRepo) Load(prefix string) (*ropa.IndexedRegoFile, error) {
	raw, err := repo.db.Get(repo.buildKey(prefix))
	if err != nil {
		return nil, err
	}

	irf := new(ropa.IndexedRegoFile)
	if err := json.Unmarshal(raw, irf); err != nil {
		return nil, err
	}

	// parsed, err := ast.ParseModule(prefix, string(raw))
	// if err != nil {
	// 	return nil, err
	// }

	return irf, nil
}
