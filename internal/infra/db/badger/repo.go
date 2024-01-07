package badger

import (
	"fmt"

	"github.com/4rchr4y/goray/internal/ropa/regom"
	"github.com/open-policy-agent/opa/ast"
)

type PolicyRepo struct {
	projPrefix string
	db         badgerClient
}

func (repo *PolicyRepo) buildKey(prefix string) []byte {
	return []byte(fmt.Sprintf("%s:policy:%s", repo.projPrefix, prefix))
}

func (repo *PolicyRepo) Store(file *regom.RegoFile) error {
	return repo.db.Set(repo.buildKey(file.Path), file.Raw)
}

func (repo *PolicyRepo) Load(prefix string) (*regom.RegoFile, error) {
	raw, err := repo.db.Get(repo.buildKey(prefix))
	if err != nil {
		return nil, err
	}

	parsed, err := ast.ParseModule(prefix, string(raw))
	if err != nil {
		return nil, err
	}

	return &regom.RegoFile{
		Path:   prefix,
		Raw:    raw,
		Parsed: parsed,
	}, nil
}
