package openpolicy

type Linker struct {
	cache  map[string]*Module
	system interface {
		ReadFile(name string) ([]byte, error)
	}
}
