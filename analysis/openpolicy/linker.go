package openpolicy

import "github.com/4rchr4y/goray/pkg/bloom"

type groupHash = string

type linker struct {
	filter *bloom.BloomFilter
	ptg    map[groupHash]int
}
