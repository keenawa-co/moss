package maps

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestMerge(t *testing.T) {
	t.Run("Merge two empty maps", func(t *testing.T) {
		m1 := map[int]string{}
		m2 := map[int]string{}
		got := Merge(m1, m2)
		want := map[int]string{}

		assert.Equal(t, want, got, "Two empty maps should merge into an empty map")
	})

	t.Run("Merge maps with distinct keys", func(t *testing.T) {
		m1 := map[int]string{1: "one"}
		m2 := map[int]string{2: "two"}
		got := Merge(m1, m2)
		want := map[int]string{1: "one", 2: "two"}

		assert.Equal(t, want, got, "Maps with distinct keys should merge correctly")
	})

	t.Run("Merge maps with overlapping keys", func(t *testing.T) {
		m1 := map[int]string{1: "one"}
		m2 := map[int]string{1: "uno", 2: "two"}
		got := Merge(m1, m2)
		want := map[int]string{1: "uno", 2: "two"}

		assert.Equal(t, want, got, "The value from the last map should overwrite the previous one")
	})

	t.Run("Merge multiple maps", func(t *testing.T) {
		m1 := map[int]string{1: "one"}
		m2 := map[int]string{2: "two"}
		m3 := map[int]string{3: "three"}
		got := Merge(m1, m2, m3)
		want := map[int]string{1: "one", 2: "two", 3: "three"}

		assert.Equal(t, want, got, "All maps should merge into one correctly")
	})

	t.Run("Merge with an empty map as one of the arguments", func(t *testing.T) {
		m1 := map[string]int{"apple": 1, "banana": 2}
		m2 := map[string]int{}
		got := Merge(m1, m2)
		want := map[string]int{"apple": 1, "banana": 2}

		assert.Equal(t, want, got, "Merging with an empty map should not change the original map")
	})

	t.Run("Merge maps with nested structures", func(t *testing.T) {
		m1 := map[string]interface{}{
			"user": map[string]string{"name": "John", "age": "30"},
		}
		m2 := map[string]interface{}{
			"user": map[string]string{"name": "John", "age": "31"}, // Update age
		}
		got := Merge(m1, m2)
		want := map[string]interface{}{
			"user": map[string]string{"name": "John", "age": "31"},
		}

		assert.Equal(t, want, got, "Nested structures should be merged correctly")
	})

	t.Run("Performance test for merging large maps", func(t *testing.T) {
		largeMap1 := make(map[int]int)
		largeMap2 := make(map[int]int)
		for i := 0; i < 100000; i++ {
			largeMap1[i] = i
			largeMap2[i+100000] = i
		}

		got := Merge(largeMap1, largeMap2)
		assert.Equal(t, 200000, len(got), "Merged map should have 200000 elements")
	})

	t.Run("Ensure all unique keys are preserved", func(t *testing.T) {
		m1 := map[string]int{"apple": 1, "banana": 2}
		m2 := map[string]int{"cherry": 3, "date": 4, "apple": 10}
		got := Merge(m1, m2)

		wantKeys := []string{"apple", "banana", "cherry", "date"}
		for _, key := range wantKeys {
			_, exists := got[key]
			assert.True(t, exists, "Expected key is missing in the merged map: "+key)
		}
	})

	t.Run("Correct value for overlapping keys (should be from the last map)", func(t *testing.T) {
		m1 := map[string]int{"apple": 1}
		m2 := map[string]int{"apple": 2}
		got := Merge(m1, m2)
		want := 2

		assert.Equal(t, want, got["apple"], "The value from the last map should overwrite the previous one for overlapping keys")
	})
}

func TestMergeFunc(t *testing.T) {
	t.Run("Merge two empty maps", func(t *testing.T) {
		m1 := map[int]string{}
		m2 := map[int]string{}
		got := MergeFunc(func(a, b string) string { return b }, m1, m2)
		want := map[int]string{}

		assert.Equal(t, want, got, "Two empty maps should merge into an empty map")
	})

	t.Run("Merge maps with distinct keys", func(t *testing.T) {
		m1 := map[int]string{1: "one"}
		m2 := map[int]string{2: "two"}
		got := MergeFunc(func(a, b string) string { return b }, m1, m2)
		want := map[int]string{1: "one", 2: "two"}

		assert.Equal(t, want, got, "Maps with distinct keys should merge correctly")
	})

	t.Run("Merge maps with overlapping keys and conflict resolution", func(t *testing.T) {
		m1 := map[int]string{1: "one"}
		m2 := map[int]string{1: "uno", 2: "two"}
		got := MergeFunc(func(a, b string) string { return a + "/" + b }, m1, m2)
		want := map[int]string{1: "one/uno", 2: "two"}

		assert.Equal(t, want, got, "The conflict should be resolved using the provided function")
	})

	t.Run("Merge multiple maps with max value for conflicts", func(t *testing.T) {
		m1 := map[int]int{1: 10, 2: 20}
		m2 := map[int]int{1: 15, 3: 25}
		m3 := map[int]int{1: 5, 4: 30}
		got := MergeFunc(func(a, b int) int {
			if a > b {
				return a
			}
			return b
		}, m1, m2, m3)
		want := map[int]int{1: 15, 2: 20, 3: 25, 4: 30}

		assert.Equal(t, want, got, "The max value should be selected for each key")
	})

	t.Run("Merge with an empty map as one of the arguments", func(t *testing.T) {
		m1 := map[string]int{"apple": 1, "banana": 2}
		m2 := map[string]int{}
		got := MergeFunc(func(a, b int) int { return b }, m1, m2)
		want := map[string]int{"apple": 1, "banana": 2}

		assert.Equal(t, want, got, "Merging with an empty map should not change the original map")
	})

	t.Run("Conflict resolution using function returning the longer string", func(t *testing.T) {
		m1 := map[string]string{"key": "short"}
		m2 := map[string]string{"key": "longer string"}
		got := MergeFunc(func(a, b string) string {
			if len(a) > len(b) {
				return a
			}
			return b
		}, m1, m2)
		want := map[string]string{"key": "longer string"}

		assert.Equal(t, want, got, "The conflict should be resolved using the function returning the longer string")
	})

	t.Run("Merge with overlapping keys choosing the last value", func(t *testing.T) {
		m1 := map[int]int{1: 100, 2: 200}
		m2 := map[int]int{1: 300, 2: 400}
		m3 := map[int]int{2: 500, 3: 600}
		got := MergeFunc(func(a, b int) int { return b }, m1, m2, m3)
		want := map[int]int{1: 300, 2: 500, 3: 600}

		assert.Equal(t, want, got, "The last value for each key should be chosen")
	})

	t.Run("Merge maps with nested structures", func(t *testing.T) {
		m1 := map[string]interface{}{
			"details": map[string]string{"name": "John", "age": "30"},
		}
		m2 := map[string]interface{}{
			"details": map[string]string{"age": "31"}, // Update age
		}
		got := MergeFunc(func(a, b interface{}) interface{} {
			ma, okA := a.(map[string]string)
			mb, okB := b.(map[string]string)
			if okA && okB {
				for k, v := range mb {
					ma[k] = v
				}
				return ma
			}
			return b
		}, m1, m2)
		want := map[string]interface{}{
			"details": map[string]string{"name": "John", "age": "31"},
		}

		assert.Equal(t, want, got, "Nested structures should be merged with custom conflict resolution")
	})
}
