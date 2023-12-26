package bloom

import (
	"hash/crc32"
	"hash/fnv"
)

type BloomFilter interface {
	Put([]byte) error
	MightContain([]byte) (bool, error)
}

// Using FNV-64a algorithm.
func hashFNV64aFunc(b []byte) uint64 {
	h := fnv.New64a()
	h.Write(b)
	return h.Sum64()
}

// Using FNV-32 algorithm.
func hashFNV32Func(b []byte) uint64 {
	h := fnv.New32()
	h.Write(b)
	return uint64(h.Sum32())
}

// Using CRC-32 algorithm.
func hashCRC32Func(b []byte) uint64 {
	h := crc32.NewIEEE()
	h.Write(b)
	return uint64(h.Sum32())
}
