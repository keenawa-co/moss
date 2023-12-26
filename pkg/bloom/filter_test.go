package bloom

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestFNV64a(t *testing.T) {
	t.Run("Should produce a non-zero hash for a non-empty input", func(t *testing.T) {
		hash := hashFNV64aFunc([]byte("hello"))
		assert.NotEqual(t, 0, hash, "Expected hashFNV64aFunc to produce a non-zero hash for 'hello'")
	})

	t.Run("Should produce the same hash for the same input consistently", func(t *testing.T) {
		hash1 := hashFNV64aFunc([]byte("hello"))
		hash2 := hashFNV64aFunc([]byte("hello"))
		assert.Equal(t, hash1, hash2, "Expected hashFNV64aFunc to produce the same hash for the same input")
	})

	t.Run("Should produce different hashes for different inputs", func(t *testing.T) {
		hash1 := hashFNV64aFunc([]byte("hello"))
		hash2 := hashFNV64aFunc([]byte("world"))
		assert.NotEqual(t, hash1, hash2, "Expected hashFNV64aFunc to produce different hashes for 'hello' and 'world'")
	})
}

func TestFNV32(t *testing.T) {
	t.Run("Should produce a non-zero hash for a non-empty input", func(t *testing.T) {
		hash := hashFNV32Func([]byte("hello"))
		assert.NotEqual(t, 0, hash, "Expected hashFNV32Func to produce a non-zero hash for 'hello'")
	})

	t.Run("Should produce the same hash for the same input consistently", func(t *testing.T) {
		hash1 := hashFNV32Func([]byte("hello"))
		hash2 := hashFNV32Func([]byte("hello"))
		assert.Equal(t, hash1, hash2, "Expected hashFNV32Func to produce the same hash for the same input")
	})

	t.Run("Should produce different hashes for different inputs", func(t *testing.T) {
		hash1 := hashFNV32Func([]byte("hello"))
		hash2 := hashFNV32Func([]byte("world"))
		assert.NotEqual(t, hash1, hash2, "Expected hashFNV32Func to produce different hashes for 'hello' and 'world'")
	})
}

func TestCRC32(t *testing.T) {
	t.Run("Should produce a non-zero hash for a non-empty input", func(t *testing.T) {
		hash := hashCRC32Func([]byte("hello"))
		assert.NotEqual(t, 0, hash, "Expected hashCRC32Func to produce a non-zero hash for 'hello'")
	})

	t.Run("Should produce the same hash for the same input consistently", func(t *testing.T) {
		hash1 := hashCRC32Func([]byte("hello"))
		hash2 := hashCRC32Func([]byte("hello"))
		assert.Equal(t, hash1, hash2, "Expected hashCRC32Func to produce the same hash for the same input")
	})

	t.Run("Should produce different hashes for different inputs", func(t *testing.T) {
		hash1 := hashCRC32Func([]byte("hello"))
		hash2 := hashCRC32Func([]byte("world"))
		assert.NotEqual(t, hash1, hash2, "Expected hashCRC32Func to produce different hashes for 'hello' and 'world'")
	})
}
