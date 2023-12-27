package openpolicy

type groupHash = string

type linker struct {
	// graph map[groupHash]
	// tgroups map[groupHash]map[int]struct{} // policy target groups
}

// func newLinker(numberOfGroups int) *linker {
// 	size, _ := bloom.CalcFilterParams(uint64(numberOfGroups), 0.01)
// 	return &linker{
// 		bfilter: bloom.NewBloomFilter(size),
// 		tgroups: make(map[string]map[int]struct{}),
// 	}
// }

func (l *linker) storeVendor(groupHash string, vendorPath string) {}

// func (l *linker) checkVendor(ghash string, vendor string) (bool, error) {
// 	// l.bfilter.MightContain([]byte(ghash))
// }
