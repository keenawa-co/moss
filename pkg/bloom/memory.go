package bloom

import (
	"math"
	"sync/atomic"
)

type memoryFilter struct {
	len       uint64
	bits      []uint64
	hashFuncs []func([]byte) uint64
}

func (filter *memoryFilter) Put(b []byte) error {
	for _, f := range filter.hashFuncs {
		val := f(b)
		idx := val % filter.len
		bitIdx := idx % 64
		arrayIdx := idx / 64

		for {
			oldVal := filter.bits[arrayIdx]
			newVal := oldVal | (1 << bitIdx)
			if atomic.CompareAndSwapUint64(&filter.bits[arrayIdx], oldVal, newVal) {
				break
			}
		}
	}
	return nil
}

func (filter *memoryFilter) MightContain(b []byte) (bool, error) {
	for _, f := range filter.hashFuncs {
		val := f(b)
		idx := val % filter.len
		bitIdx := idx % 64
		arrayIdx := idx / 64

		valAtIdx := atomic.LoadUint64(&filter.bits[arrayIdx])
		if valAtIdx&(1<<bitIdx) == 0 {
			return false, nil
		}
	}
	return true, nil
}

func NewBloomFilter(size uint64) BloomFilter {
	if size == 0 {
		size = 10000
	}
	return &memoryFilter{
		len:       size,
		bits:      make([]uint64, (size+63)/64), // Round up to nearest 64 for bit array size
		hashFuncs: []func([]byte) uint64{hashFNV64aFunc, hashFNV32Func, hashCRC32Func},
	}
}

type FilterConfig struct {
	_                        [0]int
	ExpectedItemCount        uint64
	DesiredFalsePositiveRate float64
}

func CalcFilterParams(n uint64, p float64) (uint64, int) {
	if n == 0 || p <= 0 || p >= 1 {
		return 0, 0
	}

	m := uint64(-float64(n) * math.Log(p) / math.Pow(math.Log(2), 2))
	k := int(math.Round(math.Log(2) * float64(m) / float64(n)))

	// Round up m to the nearest multiple of 64 using bitwise operation
	remainder := m % 64
	if remainder != 0 {
		m += 64 - remainder
	}

	return m, k
}
