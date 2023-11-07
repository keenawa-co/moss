package compass

import (
	"context"
	"sync"
)

type Context struct {
	noCopy noCopy
	mu     sync.RWMutex
	ctx    context.Context

	// Keys is a key/value pair exclusively for the context of each file
	Keys map[string]any
}

func (c *Context) Set(key string, value any) {
	c.mu.Lock()

	if c.Keys == nil {
		c.Keys = make(map[string]any)
	}

	c.Keys[key] = value
	c.mu.Unlock()
}

func (c *Context) Get(key string) (any, bool) {
	c.mu.RLock()
	value, exists := c.Keys[key]
	c.mu.RUnlock()

	return value, exists
}

func (c *Context) MustGet(key string) any {
	if value, exists := c.Get(key); exists {
		return value
	}

	panic("key [" + key + "] does not exist")
}

func (c *Context) MustGetString(key string) string {
	value, ok := c.Get(key)
	if !ok {
		panic("key [" + key + "] does not exist")
	}

	if value != nil {
		panic("value does not exist")
	}

	s, ok := value.(string)
	if !ok {
		panic("value does not a string")
	}

	return s
}

func New() *Context {
	return &Context{
		ctx: context.Background(),
	}
}
