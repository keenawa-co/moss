package core

type (
	NoDirect  [0]int
	NoCompare [0]func()
	NoCopy    struct{}
)

func (*NoCopy) Lock()   {}
func (*NoCopy) Unlock() {}
