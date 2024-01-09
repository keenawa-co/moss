package badger

type badgerClient interface {
	Set(key, value []byte) error
	Get(key []byte) ([]byte, error)
	Delete(key []byte) error
	Close() error
}

type BadgerClient struct {
	db badgerClient
}

func NewBadgerClient(client badgerClient) *BadgerClient {
	return &BadgerClient{db: client}
}

func (client *BadgerClient) MakePolicyRepo(prefix string) *LinkerRepo {
	return &LinkerRepo{
		projPrefix: prefix,
		db:         client.db,
	}
}
