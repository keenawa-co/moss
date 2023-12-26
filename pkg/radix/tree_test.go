package radix

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestStore(t *testing.T) {
	t.Run("Adding a new key to the tree", func(t *testing.T) {
		tree := NewTree[string]()
		oldValue, updated := tree.Store([]byte("foo"), "bar")
		assert.False(t, updated, "The tree should indicate that the key was not updated as it is new.")
		assert.Equal(t, "", oldValue, "The old value should be the zero value of the type as the key was new.")
		assert.Equal(t, 1, tree.Len(), "The tree size should be 1 after adding a new key.")
	})

	t.Run("Updating an existing key in the tree", func(t *testing.T) {
		tree := NewTree[string]()
		tree.Store([]byte("foo"), "bar")
		oldValue, updated := tree.Store([]byte("foo"), "baz")
		assert.True(t, updated, "The tree should indicate that the key was updated.")
		assert.Equal(t, "bar", oldValue, "The old value should be the previous value of the key.")
		assert.Equal(t, 1, tree.Len(), "The tree size should remain 1 after updating an existing key.")
	})

	t.Run("Adding multiple keys to the tree", func(t *testing.T) {
		tree := NewTree[string]()
		tree.Store([]byte("foo"), "bar")
		tree.Store([]byte("foobar"), "baz")
		assert.Equal(t, 2, tree.Len(), "The tree size should be 2 after adding two distinct keys.")
	})

	t.Run("Adding keys with a common prefix to the tree", func(t *testing.T) {
		tree := NewTree[string]()
		tree.Store([]byte("foo/bar"), "value1")
		oldValue, updated := tree.Store([]byte("foo/baz"), "value2")
		assert.False(t, updated, "The tree should indicate that the key was not updated as it is new, despite the common prefix.")
		assert.Equal(t, "", oldValue, "The old value should be the zero value of the type as the key was new.")
		assert.Equal(t, 2, tree.Len(), "The tree size should be 2 after adding two keys with a common prefix.")
	})

	t.Run("Adding keys with different prefixes", func(t *testing.T) {
		tree := NewTree[string]()
		tree.Store([]byte("foo"), "value1")
		tree.Store([]byte("bar"), "value2")
		assert.Equal(t, 2, tree.Len(), "The tree should correctly handle keys with different prefixes, increasing the size accordingly.")
	})

	t.Run("Adding a key that is a prefix of an existing key", func(t *testing.T) {
		tree := NewTree[string]()
		tree.Store([]byte("foobar"), "value1")
		oldValue, updated := tree.Store([]byte("foo"), "value2")
		assert.False(t, updated, "Adding a key that is a prefix of an existing key should not update the existing key and should add a new node.")
		assert.Equal(t, "", oldValue, "Expected oldValue to be default when adding a key that is a shorter prefix of an existing key.")
		assert.Equal(t, 2, tree.Len(), "The tree should correctly handle adding a key that is a prefix of an existing key.")
	})

	t.Run("Adding a key that extends an existing key", func(t *testing.T) {
		tree := NewTree[string]()
		tree.Store([]byte("foo"), "value1")
		oldValue, updated := tree.Store([]byte("foobar"), "value2")
		assert.False(t, updated, "Adding a key that extends an existing key should not update the existing key and should add a new node.")
		assert.Equal(t, "", oldValue, "Expected oldValue to be default when adding a key that extends an existing key.")
		assert.Equal(t, 2, tree.Len(), "The tree should correctly handle a key that extends an existing key.")
	})

	t.Run("Re-inserting an existing key with a different value", func(t *testing.T) {
		tree := NewTree[string]()
		tree.Store([]byte("foo"), "value1")
		oldValue, updated := tree.Store([]byte("foo"), "value2")
		assert.True(t, updated, "Re-inserting the same key with a different value should update the key's value.")
		assert.Equal(t, "value1", oldValue, "Expected oldValue to be the value before the update when re-inserting the same key.")
		assert.Equal(t, 1, tree.Len(), "The tree size should remain the same when re-inserting and updating an existing key.")
	})

	t.Run("Adding an empty key to the tree", func(t *testing.T) {
		tree := NewTree[string]()
		oldValue, updated := tree.Store([]byte(""), "value")
		assert.False(t, updated, "Adding an empty key should create a new node and should not be considered as an update.")
		assert.Equal(t, "", oldValue, "Expected oldValue to be default when adding an empty key.")
		assert.Equal(t, 1, tree.Len(), "The tree should correctly handle adding an empty key.")
	})
}

func TestRoot(t *testing.T) {
	r := NewTree[bool]()

	t.Run("Attempt to delete a non-existent root", func(t *testing.T) {
		_, ok := r.Delete([]byte(""))
		assert.False(t, ok, "Attempting to delete a non-existent root node should return false.")
	})

	t.Run("Store a value at the root", func(t *testing.T) {
		_, ok := r.Store([]byte(""), true)
		assert.False(t, ok, "Storing a value at the root for the first time should return false for updated.")
	})

	t.Run("Get the value stored at the root", func(t *testing.T) {
		val, ok := r.Load([]byte(""))
		assert.True(t, ok, "Getting the value at the root should succeed.")
		assert.True(t, val, "The value retrieved from the root should be true as stored.")
	})

	t.Run("Delete the value at the root", func(t *testing.T) {
		val, ok := r.Delete([]byte(""))
		assert.True(t, ok, "Deleting the value at the root should succeed.")
		assert.True(t, val, "The value deleted from the root should be true as it was stored.")
	})
}

func TestDelete(t *testing.T) {
	t.Run("Delete from an empty tree", func(t *testing.T) {
		tree := NewTree[string]()
		_, deleted := tree.Delete([]byte("nonexistent"))
		assert.False(t, deleted, "Deleting a non-existent key from an empty tree should return false.")
	})

	t.Run("Delete a non-existent key", func(t *testing.T) {
		tree := NewTree[string]()
		tree.Store([]byte("foo"), "bar")
		_, deleted := tree.Delete([]byte("nonexistent"))
		assert.False(t, deleted, "Deleting a non-existent key should return false.")
	})

	t.Run("Delete an existing key", func(t *testing.T) {
		tree := NewTree[string]()
		tree.Store([]byte("foo"), "bar")
		val, deleted := tree.Delete([]byte("foo"))
		assert.True(t, deleted, "Deleting an existing key should return true.")
		assert.Equal(t, "bar", val, "The returned value should match the value of the deleted key.")
		assert.Equal(t, 0, tree.Len(), "The tree should be empty after deleting the key.")
	})

	t.Run("Delete a key that is a prefix of another key", func(t *testing.T) {
		tree := NewTree[string]()
		tree.Store([]byte("foo"), "bar")
		tree.Store([]byte("foobar"), "baz")
		val, deleted := tree.Delete([]byte("foo"))
		assert.True(t, deleted, "Deleting a key that is a prefix of another key should return true.")
		assert.Equal(t, "bar", val, "The returned value should match the value of the deleted key.")
		assert.Equal(t, 1, tree.Len(), "The tree should have one remaining key after deletion.")
	})

	t.Run("Delete a key with a common prefix", func(t *testing.T) {
		tree := NewTree[string]()
		tree.Store([]byte("foo/bar"), "value1")
		tree.Store([]byte("foo/baz"), "value2")
		val, deleted := tree.Delete([]byte("foo/bar"))
		assert.True(t, deleted, "Deleting a key with a common prefix should return true.")
		assert.Equal(t, "value1", val, "The returned value should match the value of the deleted key.")
		assert.Equal(t, 1, tree.Len(), "The tree should have one remaining key after deletion.")
	})

	t.Run("Delete from a tree with a single key", func(t *testing.T) {
		tree := NewTree[string]()
		tree.Store([]byte("onlykey"), "value")
		val, deleted := tree.Delete([]byte("onlykey"))
		assert.True(t, deleted, "Deleting the only key in the tree should succeed.")
		assert.Equal(t, "value", val, "The returned value should match the value of the deleted key.")
		assert.Equal(t, 0, tree.Len(), "The tree should be empty after deleting its only key.")
	})

	t.Run("Delete a key with no common prefixes with other keys", func(t *testing.T) {
		tree := NewTree[string]()
		tree.Store([]byte("unique"), "uniqueval")
		tree.Store([]byte("other"), "otherval")
		val, deleted := tree.Delete([]byte("unique"))
		assert.True(t, deleted, "Deleting a key with no common prefixes should succeed.")
		assert.Equal(t, "uniqueval", val, "The returned value should match the value of the deleted key.")
		assert.Equal(t, 1, tree.Len(), "The tree should have one remaining key after deletion.")
	})

	t.Run("Delete a key when the node has multiple children", func(t *testing.T) {
		tree := NewTree[string]()
		tree.Store([]byte("parent"), "parentval")
		tree.Store([]byte("parent/child1"), "child1val")
		tree.Store([]byte("parent/child2"), "child2val")
		val, deleted := tree.Delete([]byte("parent"))
		assert.True(t, deleted, "Deleting a key when the node has multiple children should succeed.")
		assert.Equal(t, "parentval", val, "The returned value should match the value of the deleted key.")
		assert.Equal(t, 2, tree.Len(), "The tree should correctly handle remaining children after deletion.")
	})

	t.Run("Attempt to delete a prefix that is not a key itself", func(t *testing.T) {
		tree := NewTree[string]()
		tree.Store([]byte("prefix/key"), "value")
		_, deleted := tree.Delete([]byte("prefix"))
		assert.False(t, deleted, "Attempting to delete a prefix that is not a key itself should return false.")
		assert.Equal(t, 1, tree.Len(), "The tree size should remain unchanged when trying to delete a non-existent key.")
	})

	t.Run("Delete a key when the node has nested keys with a common prefix", func(t *testing.T) {
		tree := NewTree[string]()
		tree.Store([]byte("prefix"), "prefixval")
		tree.Store([]byte("prefix/nested"), "nestedval")
		tree.Store([]byte("prefix/nested/deep"), "deepval")
		val, deleted := tree.Delete([]byte("prefix"))
		assert.True(t, deleted, "Deleting a key when the node has nested keys with a common prefix should succeed.")
		assert.Equal(t, "prefixval", val, "The returned value should match the value of the deleted key.")
		assert.Equal(t, 2, tree.Len(), "The tree should correctly handle nested keys after deletion.")
	})
}

func TestDeletePrefix(t *testing.T) {
	t.Run("Delete nodes with prefix 'A'", func(t *testing.T) {
		tree := NewTree[bool]()
		for _, key := range [][]byte{[]byte(""), []byte("A"), []byte("AB"), []byte("ABC"), []byte("R"), []byte("S")} {
			tree.Store(key, true)
		}
		deleted := tree.DeletePrefix([]byte("A"))
		assert.Equal(t, 3, deleted, "Expected 3 nodes to be deleted")

		out := [][]byte{}
		tree.Walk(func(s []byte, v bool) bool {
			out = append(out, s)
			return false
		})
		assert.ElementsMatch(t, [][]byte{[]byte(""), []byte("R"), []byte("S")}, out, "Expected remaining nodes after deletion")
	})

	t.Run("Delete nodes with prefix 'ABC'", func(t *testing.T) {
		tree := NewTree[bool]()
		for _, key := range [][]byte{[]byte(""), []byte("A"), []byte("AB"), []byte("ABC"), []byte("R"), []byte("S")} {
			tree.Store(key, true)
		}
		deleted := tree.DeletePrefix([]byte("ABC"))
		assert.Equal(t, 1, deleted, "Expected 1 node to be deleted")

		out := [][]byte{}
		tree.Walk(func(s []byte, v bool) bool {
			out = append(out, s)
			return false
		})
		assert.ElementsMatch(t, [][]byte{[]byte(""), []byte("A"), []byte("AB"), []byte("R"), []byte("S")}, out, "Expected remaining nodes after deletion")
	})

	t.Run("Delete all nodes", func(t *testing.T) {
		tree := NewTree[bool]()
		for _, key := range [][]byte{[]byte(""), []byte("A"), []byte("AB"), []byte("ABC"), []byte("R"), []byte("S")} {
			tree.Store(key, true)
		}
		deleted := tree.DeletePrefix([]byte(""))
		assert.Equal(t, 6, deleted, "Expected all nodes to be deleted")

		out := [][]byte{}
		tree.Walk(func(s []byte, v bool) bool {
			out = append(out, s)
			return false
		})
		assert.Empty(t, out, "Expected no remaining nodes after deletion")
	})

	t.Run("Delete nodes with prefix 'S'", func(t *testing.T) {
		tree := NewTree[bool]()
		for _, key := range [][]byte{[]byte(""), []byte("A"), []byte("AB"), []byte("ABC"), []byte("R"), []byte("S")} {
			tree.Store(key, true)
		}
		deleted := tree.DeletePrefix([]byte("S"))
		assert.Equal(t, 1, deleted, "Expected 1 node to be deleted")

		out := [][]byte{}
		tree.Walk(func(s []byte, v bool) bool {
			out = append(out, s)
			return false
		})
		assert.ElementsMatch(t, [][]byte{[]byte(""), []byte("A"), []byte("AB"), []byte("ABC"), []byte("R")}, out, "Expected remaining nodes after deletion")
	})

	t.Run("Attempt to delete non-existent prefix 'SS'", func(t *testing.T) {
		tree := NewTree[bool]()
		for _, key := range [][]byte{[]byte(""), []byte("A"), []byte("AB"), []byte("ABC"), []byte("R"), []byte("S")} {
			tree.Store(key, true)
		}
		deleted := tree.DeletePrefix([]byte("SS"))
		assert.Equal(t, 0, deleted, "Expected no nodes to be deleted for non-existent prefix")

		out := [][]byte{}
		tree.Walk(func(s []byte, v bool) bool {
			out = append(out, s)
			return false
		})
		assert.ElementsMatch(t, [][]byte{[]byte(""), []byte("A"), []byte("AB"), []byte("ABC"), []byte("R"), []byte("S")}, out, "Expected all nodes to remain after attempting to delete non-existent prefix")
	})
}

func TestLongestPrefix(t *testing.T) {
	r := NewTree[bool]()

	keys := [][]byte{
		[]byte(""),
		[]byte("foo"),
		[]byte("foobar"),
		[]byte("foobarbaz"),
		[]byte("foobarbazzip"),
		[]byte("foozip"),
	}
	for _, k := range keys {
		r.Store(k, true)
	}
	assert.Equal(t, len(keys), r.Len(), "The tree should have the correct number of keys.")

	t.Run("Input shorter than any key", func(t *testing.T) {
		leaf, ok := r.LongestPrefix([]byte("a"))
		assert.True(t, ok, "There should always be a match for any input.")
		assert.Equal(t, "", string(leaf.Key), "The longest prefix of 'a' should be the root key.")
	})

	t.Run("Input with no direct match", func(t *testing.T) {
		leaf, ok := r.LongestPrefix([]byte("abc"))
		assert.True(t, ok, "There should always be a match for any input.")
		assert.Equal(t, "", string(leaf.Key), "The longest prefix of 'abc' should be the root key.")
	})

	t.Run("Input that is a prefix of stored keys", func(t *testing.T) {
		leaf, ok := r.LongestPrefix([]byte("fo"))
		assert.True(t, ok, "There should always be a match for 'fo'.")
		assert.Equal(t, "", string(leaf.Key), "The longest prefix of 'fo' should be the root key.")
	})

	t.Run("Exact match with a key", func(t *testing.T) {
		leaf, ok := r.LongestPrefix([]byte("foo"))
		assert.True(t, ok, "There should be a match for 'foo'.")
		assert.Equal(t, "foo", string(leaf.Key), "The longest prefix of 'foo' should be 'foo'.")
	})

	t.Run("Input longer than a key but with common prefix", func(t *testing.T) {
		leaf, ok := r.LongestPrefix([]byte("foobarba"))
		assert.True(t, ok, "There should be a match for 'foobarba'.")
		assert.Equal(t, "foobar", string(leaf.Key), "The longest prefix of 'foobarba' should be 'foobar'.")
	})

	t.Run("Exact match with the longest key", func(t *testing.T) {
		leaf, ok := r.LongestPrefix([]byte("foobarbazzip"))
		assert.True(t, ok, "There should be a match for 'foobarbazzip'.")
		assert.Equal(t, "foobarbazzip", string(leaf.Key), "The longest prefix of 'foobarbazzip' should be itself.")
	})

	t.Run("Input with common prefix but not an exact match", func(t *testing.T) {
		leaf, ok := r.LongestPrefix([]byte("foozipzap"))
		assert.True(t, ok, "There should be a match for 'foozipzap'.")
		assert.Equal(t, "foozip", string(leaf.Key), "The longest prefix of 'foozipzap' should be 'foozip'.")
	})
}

func TestWalkPrefix(t *testing.T) {
	tree := NewTree[string]()

	keys := [][]byte{
		[]byte("foobar"),
		[]byte("foo/bar/baz"),
		[]byte("foo/baz/bar"),
		[]byte("foo/zip/zap"),
		[]byte("zipzap"),
	}

	for _, k := range keys {
		tree.Store(k, "")
	}

	assert.Equal(t, len(keys), tree.Len(), "The tree should have the correct number of keys.")

	t.Run("Walk with prefix 'f'", func(t *testing.T) {
		expected := []string{"foobar", "foo/bar/baz", "foo/baz/bar", "foo/zip/zap"}
		out := []string{}
		tree.WalkPrefix([]byte("f"), func(s []byte, v string) bool {
			out = append(out, string(s))
			return false
		})
		sort.Strings(out)
		assert.ElementsMatch(t, expected, out, "The walked paths should match expected for prefix 'f'")
	})

	t.Run("Walk with prefix 'foo'", func(t *testing.T) {
		expected := []string{"foobar", "foo/bar/baz", "foo/baz/bar", "foo/zip/zap"}
		out := []string{}
		tree.WalkPrefix([]byte("foo"), func(s []byte, v string) bool {
			out = append(out, string(s))
			return false
		})
		sort.Strings(out)
		assert.ElementsMatch(t, expected, out, "The walked paths should match expected for prefix 'foo'")
	})

	t.Run("Walk with prefix 'foob'", func(t *testing.T) {
		expected := []string{"foobar"}
		out := []string{}
		tree.WalkPrefix([]byte("foob"), func(s []byte, v string) bool {
			out = append(out, string(s))
			return false
		})
		sort.Strings(out)
		assert.ElementsMatch(t, expected, out, "The walked paths should match expected for prefix 'foob'")
	})

	t.Run("Walk with prefix 'foo/'", func(t *testing.T) {
		expected := []string{"foo/bar/baz", "foo/baz/bar", "foo/zip/zap"}
		out := []string{}
		tree.WalkPrefix([]byte("foo/"), func(s []byte, v string) bool {
			out = append(out, string(s))
			return false
		})
		sort.Strings(out)
		assert.ElementsMatch(t, expected, out, "The walked paths should match expected for prefix 'foo/'")
	})

	t.Run("Walk with exact prefix 'foo/bar/baz'", func(t *testing.T) {
		expected := []string{"foo/bar/baz"}
		out := []string{}
		tree.WalkPrefix([]byte("foo/bar/baz"), func(s []byte, v string) bool {
			out = append(out, string(s))
			return false
		})
		sort.Strings(out)
		assert.ElementsMatch(t, expected, out, "The walked paths should match expected for exact prefix 'foo/bar/baz'")
	})

	t.Run("Walk with non-existent prefix 'foo/bar/bazoo'", func(t *testing.T) {
		expected := []string{}
		out := []string{}
		tree.WalkPrefix([]byte("foo/bar/bazoo"), func(s []byte, v string) bool {
			out = append(out, string(s))
			return false
		})
		sort.Strings(out)
		assert.ElementsMatch(t, expected, out, "The walked paths should match expected for non-existent prefix 'foo/bar/bazoo'")
	})

	t.Run("Walk with prefix 'z'", func(t *testing.T) {
		expected := []string{"zipzap"}
		out := []string{}
		tree.WalkPrefix([]byte("z"), func(s []byte, v string) bool {
			out = append(out, string(s))
			return false
		})
		sort.Strings(out)
		assert.ElementsMatch(t, expected, out, "The walked paths should match expected for prefix 'z'")
	})

}

func TestWalkPath(t *testing.T) {
	r := NewTree[string]()

	keys := [][]byte{
		[]byte("foo"),
		[]byte("foo/bar"),
		[]byte("foo/bar/baz"),
		[]byte("foo/baz/bar"),
		[]byte("foo/zip/zap"),
		[]byte("zipzap"),
	}
	for _, k := range keys {
		r.Store(k, "")
	}

	assert.Equal(t, len(keys), r.Len(), "The tree should have the correct number of keys.")

	t.Run("Walk path with prefix 'f'", func(t *testing.T) {
		expected := []string{}
		out := []string{}
		r.WalkPath([]byte("f"), func(s []byte, v string) bool {
			out = append(out, string(s))
			return false
		})
		sort.Strings(out)
		assert.ElementsMatch(t, expected, out, "The walked paths should match expected for prefix 'f'")
	})

	t.Run("Walk path with prefix 'foo'", func(t *testing.T) {
		expected := []string{"foo"}
		out := []string{}
		r.WalkPath([]byte("foo"), func(s []byte, v string) bool {
			out = append(out, string(s))
			return false
		})
		sort.Strings(out)
		assert.ElementsMatch(t, expected, out, "The walked paths should match expected for prefix 'foo'")
	})

	t.Run("Walk path with prefix 'foo/'", func(t *testing.T) {
		expected := []string{"foo"}
		out := []string{}
		r.WalkPath([]byte("foo/"), func(s []byte, v string) bool {
			out = append(out, string(s))
			return false
		})
		sort.Strings(out)
		assert.ElementsMatch(t, expected, out, "The walked paths should match expected for prefix 'foo/'")
	})

	t.Run("Walk path with prefix 'foo/ba'", func(t *testing.T) {
		expected := []string{"foo"}
		out := []string{}
		r.WalkPath([]byte("foo/ba"), func(s []byte, v string) bool {
			out = append(out, string(s))
			return false
		})
		sort.Strings(out)
		assert.ElementsMatch(t, expected, out, "The walked paths should match expected for prefix 'foo/ba'")
	})

	t.Run("Walk path with prefix 'foo/bar'", func(t *testing.T) {
		expected := []string{"foo", "foo/bar"}
		out := []string{}
		r.WalkPath([]byte("foo/bar"), func(s []byte, v string) bool {
			out = append(out, string(s))
			return false
		})
		sort.Strings(out)
		assert.ElementsMatch(t, expected, out, "The walked paths should match expected for prefix 'foo/bar'")
	})

	t.Run("Walk path with exact prefix 'foo/bar/baz'", func(t *testing.T) {
		expected := []string{"foo", "foo/bar", "foo/bar/baz"}
		out := []string{}
		r.WalkPath([]byte("foo/bar/baz"), func(s []byte, v string) bool {
			out = append(out, string(s))
			return false
		})
		sort.Strings(out)
		assert.ElementsMatch(t, expected, out, "The walked paths should match expected for exact prefix 'foo/bar/baz'")
	})

	t.Run("Walk path with non-existent prefix 'z'", func(t *testing.T) {
		expected := []string{}
		out := []string{}
		r.WalkPath([]byte("z"), func(s []byte, v string) bool {
			out = append(out, string(s))
			return false
		})
		sort.Strings(out)
		assert.ElementsMatch(t, expected, out, "The walked paths should match expected for non-existent prefix 'z'")
	})
}

func TestWalkDelete(t *testing.T) {
	r := NewTree[bool]()

	keys := [][]byte{
		[]byte("init0/0"),
		[]byte("init0/1"),
		[]byte("init0/2"),
		[]byte("init0/3"),
		[]byte("init1/0"),
		[]byte("init1/1"),
		[]byte("init1/2"),
		[]byte("init1/3"),
		[]byte("init2"),
	}

	for _, k := range keys {
		r.Store(k, false)
	}

	assert.Equal(t, len(keys), r.Len(), "Initial tree should have the correct number of keys.")

	deleteFn := func(s []byte, v bool) bool {
		r.Delete(s)
		return false
	}

	t.Run("Delete nodes with prefix 'init1'", func(t *testing.T) {
		r.WalkPrefix([]byte("init1"), deleteFn)

		for _, s := range []string{"init0/0", "init0/1", "init0/2", "init0/3", "init2"} {
			_, ok := r.Load([]byte(s))
			assert.True(t, ok, "Expected to still find key "+s)
		}
		assert.Equal(t, 5, r.Len(), "Expected to find exactly 5 nodes after deletion")
	})

	t.Run("Delete all remaining nodes", func(t *testing.T) {
		r.Walk(deleteFn)
		assert.Equal(t, 0, r.Len(), "Expected to find exactly 0 nodes after deleting all")
	})
}

func TestLoad(t *testing.T) {
	tree := Tree[string]{Root: &Node[string]{}}

	tree.Store([]byte("key1"), "value1")
	tree.Store([]byte("key2"), "value2")
	tree.Store([]byte("prefix"), "prefix_value")

	t.Run("Load existing value", func(t *testing.T) {
		value, found := tree.Load([]byte("key1"))
		assert.True(t, found, "The value should be found in the tree")
		assert.Equal(t, "value1", value, "The loaded value should match the stored value")
	})

	t.Run("Load non-existing value", func(t *testing.T) {
		_, found := tree.Load([]byte("nonexistent"))
		assert.False(t, found, "Non-existing value should not be found in the tree")
	})

	t.Run("Load value with common prefix", func(t *testing.T) {
		value, found := tree.Load([]byte("prefix"))
		assert.True(t, found, "The value with a common prefix should be found in the tree")
		assert.Equal(t, "prefix_value", value, "The loaded value should match the stored value")
	})

	t.Run("Load value with partial key match", func(t *testing.T) {
		_, found := tree.Load([]byte("key"))
		assert.False(t, found, "Partial key match should not return a value")
	})

	t.Run("Load value with longer key than stored", func(t *testing.T) {
		_, found := tree.Load([]byte("key1_extra"))
		assert.False(t, found, "Key longer than stored key should not return a value")
	})
}

func TestLoadPrefix(t *testing.T) {
	tree := NewTree[string]()

	tree.Store([]byte("home"), "Policy 1 Data")
	tree.Store([]byte("opa/lib/go/ast/types.go"), "Types Data")
	tree.Store([]byte("opa/lib/go/ast/tokens.go"), "Tokens Data")
	tree.Store([]byte("opa/lib/go/ast/kinds.go"), "Kinds Data")
	tree.Store([]byte("opa/policy/r1.go"), "Policy R1 Data")
	tree.Store([]byte("opa/policy/r2.go"), "Policy R2 Data")

	t.Run("LoadPrefix with valid prefix 'home'", func(t *testing.T) {
		leaves, ok := tree.LoadPrefix([]byte("home"))
		assert.True(t, ok, "Expected to find prefix")
		assert.Len(t, leaves, 1, "Expected one leaf")
		assert.Equal(t, "Policy 1 Data", leaves[0].Value, "Unexpected leaf data")
	})

	t.Run("LoadPrefix with valid deep prefix 'opa/lib'", func(t *testing.T) {
		leaves, ok := tree.LoadPrefix([]byte("opa/lib"))
		assert.True(t, ok, "Expected to find prefix")
		assert.Len(t, leaves, 3)
		expectedValues := map[string]bool{"Types Data": false, "Tokens Data": false, "Kinds Data": false}
		for _, leaf := range leaves {
			_, exists := expectedValues[leaf.Value]
			assert.True(t, exists, "Unexpected leaf data")
			expectedValues[leaf.Value] = true
		}
		for _, found := range expectedValues {
			assert.True(t, found, "Did not find all expected leaf data")
		}
	})

	t.Run("LoadPrefix with non-existent prefix 'missing'", func(t *testing.T) {
		leaves, ok := tree.LoadPrefix([]byte("missing"))
		assert.False(t, ok, "Expected not to find prefix")
		assert.Nil(t, leaves, "Expected no leaves to be returned")
	})

	t.Run("LoadPrefix with edge case prefix 'opa/policy/'", func(t *testing.T) {
		leaves, ok := tree.LoadPrefix([]byte("opa/policy/"))
		assert.True(t, ok, "Expected to find prefix")
		assert.Len(t, leaves, 2)
		expectedValues := map[string]bool{"Policy R1 Data": false, "Policy R2 Data": false}
		for _, leaf := range leaves {
			_, exists := expectedValues[leaf.Value]
			assert.True(t, exists, "Unexpected leaf data")
			expectedValues[leaf.Value] = true
		}
		for _, found := range expectedValues {
			assert.True(t, found, "Did not find all expected leaf data")
		}
	})
}
