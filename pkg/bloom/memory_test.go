package bloom

import (
	"crypto/rand"
	"sync"
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestPutAndMightContain(t *testing.T) {
	filter := NewBloomFilter(1000)

	// Test putting a valid item into the Bloom filter.
	t.Run("Put valid item", func(t *testing.T) {
		err := filter.Put([]byte("valid_item"))
		require.NoError(t, err, "Inserting an item should not produce an error")
	})

	// Test checking the presence of an item that was added to the filter.
	t.Run("MightContain with existing item", func(t *testing.T) {
		contains, err := filter.MightContain([]byte("valid_item"))
		require.NoError(t, err, "Checking for an existing item should not produce an error")
		require.True(t, contains, "The filter should indicate the presence of an item that was added")
	})

	// Test checking the presence of an item that was not added to the filter.
	t.Run("MightContain with non-existing item", func(t *testing.T) {
		contains, err := filter.MightContain([]byte("non_existent_item"))
		require.NoError(t, err, "Checking for a non-existing item should not produce an error")
		require.False(t, contains, "The filter should not indicate the presence of an item that was not added")
	})

	// Test checking the presence of an empty string.
	t.Run("MightContain with empty item", func(t *testing.T) {
		contains, err := filter.MightContain([]byte(""))
		require.NoError(t, err, "Checking for an empty item should not produce an error")
		require.False(t, contains, "The filter should not indicate the presence of an empty item by default")
	})

	// Test checking the presence of a nil value.
	t.Run("MightContain with nil item", func(t *testing.T) {
		contains, err := filter.MightContain(nil)
		require.NoError(t, err, "Checking for a nil item should not produce an error")
		require.False(t, contains, "The filter should not indicate the presence of a nil item by default")
	})
}

func TestConcurrentAccess(t *testing.T) {
	filter := NewBloomFilter(10000)
	n := 1000

	t.Run("Put valid item", func(t *testing.T) {
		var wg sync.WaitGroup
		wg.Add(n)

		for i := 0; i < n; i++ {
			go func() {
				defer wg.Done()
				b := make([]byte, 10)
				_, err := rand.Read(b)
				require.NoError(t, err, "Random data generation shouldn't fail")

				err = filter.Put(b)
				require.NoError(t, err, "Put should succeed without error")
			}()
		}

		wg.Wait()
	})

	t.Run("Concurrent might contain valid", func(t *testing.T) {
		var wg sync.WaitGroup
		wg.Add(n)

		for i := 0; i < n; i++ {
			go func() {
				defer wg.Done()
				b := make([]byte, 10)
				_, err := rand.Read(b)
				require.NoError(t, err, "Random data generation shouldn't fail")

				err = filter.Put(b)
				require.NoError(t, err, "Put should succeed without error")

				contains, err := filter.MightContain(b)
				require.NoError(t, err, "MightContain should succeed without error")
				require.True(t, contains, "Item should be in filter")
			}()
		}

		wg.Wait()
	})

	t.Run("Concurrent might contain invalid", func(t *testing.T) {
		var wg sync.WaitGroup
		wg.Add(n)

		for i := 0; i < n; i++ {
			go func() {
				defer wg.Done()
				b := make([]byte, 10)
				_, err := rand.Read(b)
				require.NoError(t, err, "Random data generation shouldn't fail")

				contains, err := filter.MightContain(b)
				require.NoError(t, err, "MightContain should succeed without error")
				if contains {
					t.Logf("False positive: Item was detected in filter but should not be")
				}
			}()
		}

		wg.Wait()
	})
}

func TestCalcFilterParams(t *testing.T) {
	t.Run("Valid with K 3", func(t *testing.T) {
		m, k := CalcFilterParams(5000, 0.1)
		expectedM, expectedK := uint64(24000), 3

		assert.Equal(t, expectedM, m, "M should match the expected value for K=3")
		assert.Equal(t, expectedK, k, "K should match the expected value for K=3")
	})

	t.Run("Valid with K 7", func(t *testing.T) {
		m, k := CalcFilterParams(1000, 0.01)
		expectedM, expectedK := uint64(9600), 7

		assert.Equal(t, expectedM, m, "M should match the expected value for K=7")
		assert.Equal(t, expectedK, k, "K should match the expected value for K=7")
	})

	t.Run("Zero items", func(t *testing.T) {
		m, k := CalcFilterParams(0, 0.01)
		assert.Equal(t, uint64(0), m, "M should be 0 when the number of items is 0")
		assert.Equal(t, 0, k, "K should be 0 when the number of items is 0")
	})

	t.Run("Invalid probability low", func(t *testing.T) {
		m, k := CalcFilterParams(1000, -0.01)
		assert.Equal(t, uint64(0), m, "M should be 0 for negative probability")
		assert.Equal(t, 0, k, "K should be 0 for negative probability")
	})

	t.Run("Invalid probability high", func(t *testing.T) {
		m, k := CalcFilterParams(1000, 1.5)
		assert.Equal(t, uint64(0), m, "M should be 0 for probabilities greater than 1")
		assert.Equal(t, 0, k, "K should be 0 for probabilities greater than 1")
	})
}
