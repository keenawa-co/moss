package openpolicy

import "github.com/dgraph-io/badger/v4"

type Store struct {
	db *badger.DB
}
