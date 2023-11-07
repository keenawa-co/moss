package compass

import "sync"

type Compass struct {
	noCopy noCopy
	mu     sync.RWMutex
	once   sync.Once

	engine *Engine
}

func NewCompass() *Compass {
	return &Compass{}
}
