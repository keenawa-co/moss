package compass

import (
	"sync"

	"github.com/4rchr4y/go-compass/obj"
	"golang.org/x/mod/modfile"
)

type State struct {
	noCopy noCopy

	mu   sync.RWMutex
	once sync.Once

	// storage is a key/value pair exclusively for the context of each file
	storage map[string]any

	// File is a data structure for the analyzed file
	File    *obj.FileObj
	Modfile *modfile.File
}

func (s *State) Set(key string, value any) {
	s.mu.Lock()

	if s.storage == nil {
		s.once.Do(func() {
			s.storage = make(map[string]any)
		})
	}

	s.storage[key] = value
	s.mu.Unlock()
}

func (s *State) Get(key string) (any, bool) {
	s.mu.RLock()
	value, exists := s.storage[key]
	s.mu.RUnlock()

	return value, exists
}

func (s *State) MustGet(key string) any {
	if value, exists := s.Get(key); exists {
		return value
	}

	panic("key [" + key + "] does not exist")
}

func (s *State) MustGetString(key string) string {
	value, ok := s.Get(key)
	if !ok {
		panic("key [" + key + "] does not exist")
	}

	if value != nil {
		panic("value does not exist")
	}

	str, ok := value.(string)
	if !ok {
		panic("value does not a string")
	}

	return str
}
