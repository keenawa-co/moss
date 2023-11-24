package goray

type (
	noDirect  [0]int
	noCompare [0]func()
	noCopy    struct{}
)

func (*noCopy) Lock()   {}
func (*noCopy) Unlock() {}
