package badger

import (
	"github.com/dgraph-io/badger/v4"
)

type badgerDB interface {
	Update(fn func(txn *badger.Txn) error) error
	View(fn func(txn *badger.Txn) error) error
	Close() error
}

type BadgerDB struct {
	db badgerDB
}

func NewBadgerDB(dbPath string) (*BadgerDB, error) {
	opts := badger.DefaultOptions(dbPath)
	opts.Logger = nil

	db, err := badger.Open(opts)
	if err != nil {
		return nil, err
	}

	return &BadgerDB{
		db: db,
	}, nil
}

func (client *BadgerDB) Set(key, value []byte) error {
	return client.db.Update(func(txn *badger.Txn) error {
		return txn.Set(key, value)
	})
}

func (client *BadgerDB) Get(key []byte) ([]byte, error) {
	var valCopy []byte

	err := client.db.View(func(txn *badger.Txn) error {
		item, err := txn.Get(key)
		if err != nil {
			return err
		}

		valCopy, err = item.ValueCopy(valCopy)
		return err
	})

	return valCopy, err
}

// func (client *badgerDB) GetByPrefix(prefix []byte) (map[string][]byte, error) {
// 	result := make(map[string][]byte)

// 	err := client.db.View(func(txn *badger.Txn) error {
// 		opts := badger.DefaultIteratorOptions
// 		opts.Prefix = prefix
// 		it := txn.NewIterator(opts)
// 		defer it.Close()

// 		for it.Seek(prefix); it.ValidForPrefix(prefix); it.Next() {
// 			item := it.Item()
// 			key := item.KeyCopy(nil)
// 			val, err := item.ValueCopy(nil)
// 			if err != nil {
// 				return err
// 			}

// 			result[string(key)] = val
// 		}
// 		return nil
// 	})

// 	return result, err
// }

func (client *BadgerDB) Delete(key []byte) error {
	return client.db.Update(func(txn *badger.Txn) error {
		return txn.Delete(key)
	})
}

func (client *BadgerDB) Close() error {
	return client.db.Close()
}
