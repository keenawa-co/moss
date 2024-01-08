package radix

import (
	"bytes"
)

type WalkFn[T any] func(key []byte, value T) bool

type Tree[T any] struct {
	Root *Node[T]
	Size int
}

func NewTree[V any]() *Tree[V] {
	return &Tree[V]{
		Root: &Node[V]{},
	}
}

func (t *Tree[V]) Len() int {
	return t.Size
}

func (t *Tree[T]) Store(key []byte, value T) (old T, updated bool) {
	var parent *Node[T]
	n := t.Root
	search := key

	for {
		if len(search) == 0 {
			if n.isLeaf() {
				old = n.Leaf.Value
				n.Leaf.Value = value
				return old, true
			}

			n.Leaf = &Leaf[T]{Key: key, Value: value}
			t.Size++
			return old, false
		}

		parent = n
		n = n.getEdge(search[0])

		if n == nil {
			e := &Edge[T]{
				Label: search[0],
				Node: &Node[T]{
					Leaf:   &Leaf[T]{Key: key, Value: value},
					Prefix: search,
				},
			}

			parent.addEdge(e)
			t.Size++
			return old, false
		}

		commonPrefix := longestPrefix(search, n.Prefix)
		if commonPrefix == len(n.Prefix) {
			search = search[commonPrefix:]
			continue
		}

		t.Size++
		child := &Node[T]{
			Prefix: search[:commonPrefix],
		}
		parent.updateEdge(search[0], child)

		child.addEdge(&Edge[T]{Label: n.Prefix[commonPrefix], Node: n})
		n.Prefix = n.Prefix[commonPrefix:]

		if len(search) == commonPrefix {
			child.Leaf = &Leaf[T]{Key: key, Value: value}
			return old, false
		}

		child.addEdge(&Edge[T]{
			Label: search[commonPrefix],
			Node: &Node[T]{
				Leaf:   &Leaf[T]{Key: key, Value: value},
				Prefix: search[commonPrefix:],
			},
		})
		return old, false
	}
}

func (t *Tree[V]) Load(key []byte) (value V, found bool) {
	n := t.Root
	search := key

	for n != nil {
		if len(search) == 0 {
			if n.isLeaf() {
				return n.Leaf.Value, true // Return immediately if a leaf node is found.
			}

			return // key is exhausted but not a leaf node.
		}

		n = n.getEdge(search[0])
		if n == nil {
			return // no further edge is found.
		}

		if !bytes.HasPrefix(search, n.Prefix) {
			return // current node's prefix does not match.
		}

		search = bytes.Clone(search[len(n.Prefix):]) // proceed with the remaining part of the key.
	}
	return
}

func (t *Tree[T]) LoadPrefix(prefix []byte) (map[string]T, bool) {
	n := t.Root
	search := prefix

	for len(search) > 0 {
		if n == nil {
			return nil, false
		}

		// getting the next node by label
		n = n.getEdge(search[0])
		if n == nil {
			return nil, false
		}

		// node prefix matches the one we are looking for, continue the search inside this node
		if bytes.HasPrefix(search, n.Prefix) {
			search = bytes.Clone(search[len(n.Prefix):])
			continue
		}

		// the searched prefix coincides with the beginning of the node prefix
		if bytes.HasPrefix(n.Prefix, search) {
			break
		}

		return nil, false
	}

	walker := func(key []byte, value T) bool { return bytes.HasPrefix(key, prefix) }
	leaves := collectLeaves(n, walker)

	if len(leaves) > 0 {
		result := make(map[string]T, len(leaves))

		for _, leaf := range leaves {
			result[string(leaf.Key)] = leaf.Value
		}

		return result, true
	}

	return nil, false
}

// Delete removes a key from the tree and returns the deleted
// value and a boolean indicating successful deletion
func (t *Tree[T]) Delete(key []byte) (value T, deleted bool) {
	var parent *Node[T]
	var label byte
	n := t.Root
	search := key

	// traverse down the tree to find the node matching the key
	for len(search) > 0 {
		if n == nil {
			return value, false // node not found
		}

		parent = n
		label = search[0]
		n = n.getEdge(label) // get the next node based on the edge label
		if n == nil {
			return value, false // edge not found
		}

		if !bytes.HasPrefix(search, n.Prefix) {
			return value, false // prefix doesn't match
		}

		search = bytes.Clone(search[len(n.Prefix):]) // update the search term
	}

	if !n.isLeaf() {
		return value, false // node is not a leaf, can't delete non-leaf nodes
	}

	// perform deletion
	value = n.Leaf.Value
	n.Leaf = nil
	t.Size--
	deleted = true

	if parent != nil && len(n.Edges) == 0 {
		parent.delEdge(label) // remove the edge from the parent if the current node has no more children
	}

	if n != t.Root && len(n.Edges) == 1 {
		n.mergeChild() // merge the node with its single child
	}

	if parent != nil && parent != t.Root && len(parent.Edges) == 1 && !parent.isLeaf() {
		parent.mergeChild() // merge the parent with its single child
	}

	return value, deleted
}

func (t *Tree[T]) DeletePrefix(prefix []byte) int {
	return t.deletePrefix(nil, t.Root, prefix)
}

func (t *Tree[T]) deletePrefix(parent, node *Node[T], prefix []byte) int {
	if len(prefix) == 0 {
		return t.handleFullPrefixDeletion(node, parent)
	}

	label := prefix[0]
	child := node.getEdge(label) // retrieve next node in the path
	if child == nil || (!bytes.HasPrefix(child.Prefix, prefix) && !bytes.HasPrefix(prefix, child.Prefix)) {
		return 0 // no matching child is found
	}

	// adjust the prefix for deeper recursion based on the matched child node's prefix
	if len(child.Prefix) > len(prefix) {
		prefix = prefix[len(prefix):]
	} else {
		prefix = prefix[len(child.Prefix):]
	}
	return t.deletePrefix(node, child, prefix) // recursive call to continue deletion process
}

func (t *Tree[T]) handleFullPrefixDeletion(node, parent *Node[T]) int {
	counter := 0 // number of nodes to be deleted

	// walk the subtree to count and delete nodes
	recursiveWalk(node, func(s []byte, v T) bool {
		counter++
		return false
	})

	// clear the current node's data and links
	if node.isLeaf() {
		node.Leaf = nil
	}

	node.Edges = nil

	// merge parent's child node if certain conditions are met
	if parent != nil && parent != t.Root && len(parent.Edges) == 1 && !parent.isLeaf() {
		parent.mergeChild()
	}
	t.Size -= counter // update tree size.
	return counter
}

func (t *Tree[T]) Minimum() (*Node[T], bool) {
	n := t.Root
	for {
		if n.isLeaf() {
			return n, true
		}
		if len(n.Edges) > 0 {
			n = n.Edges[0].Node
		} else {
			break
		}
	}
	return nil, false
}

func (t *Tree[T]) Maximum() (*Node[T], bool) {
	n := t.Root
	for {
		if num := len(n.Edges); num > 0 {
			n = n.Edges[num-1].Node
			continue
		}
		if n.isLeaf() {
			return n, true
		}
		break
	}
	return nil, false
}

func (t *Tree[T]) Walk(fn WalkFn[T]) {
	recursiveWalk(t.Root, fn)
}

func (t *Tree[T]) WalkPrefix(prefix []byte, fn WalkFn[T]) {
	n := t.Root
	search := prefix
	for {
		if len(search) == 0 {
			recursiveWalk(n, fn)
			return
		}

		n = n.getEdge(search[0])
		if n == nil {
			return
		}

		if bytes.HasPrefix(search, n.Prefix) {
			search = search[len(n.Prefix):]
			continue
		}
		if bytes.HasPrefix(n.Prefix, search) {
			recursiveWalk(n, fn)
		}
		return
	}
}

func (t *Tree[T]) WalkPath(path []byte, fn WalkFn[T]) {
	n := t.Root
	search := path

	for {
		if n.Leaf != nil && fn(n.Leaf.Key, n.Leaf.Value) {
			return
		}

		if len(search) == 0 {
			return
		}

		n = n.getEdge(search[0])
		if n == nil {
			return
		}

		if bytes.HasPrefix(search, n.Prefix) {
			search = search[len(n.Prefix):]
		} else {
			break
		}
	}
}

func (t *Tree[T]) LongestPrefix(key []byte) (*Leaf[T], bool) {
	var last *Leaf[T]
	n := t.Root
	search := key

	for {
		if n.isLeaf() {
			last = n.Leaf
		}

		if len(search) == 0 {
			break
		}

		n = n.getEdge(search[0])
		if n == nil {
			break
		}

		if bytes.HasPrefix(search, n.Prefix) {
			search = search[len(n.Prefix):]
		} else {
			break
		}
	}
	if last != nil {
		return last, true
	}

	return nil, false
}

func recursiveWalk[T any](node *Node[T], fn WalkFn[T]) bool {
	if node.Leaf != nil && fn(node.Leaf.Key, node.Leaf.Value) {
		return true
	}

	i, k := 0, len(node.Edges)
	for i < k {
		e := node.Edges[i]

		if recursiveWalk[T](e.Node, fn) {
			return true
		}

		if len(node.Edges) == 0 {
			return recursiveWalk(node, fn)
		}

		if len(node.Edges) >= k {
			i++
		}

		k = len(node.Edges)
	}
	return false
}

func collectLeaves[T any](node *Node[T], walk WalkFn[T]) []*Leaf[T] {
	var leaves []*Leaf[T]

	if node.Leaf != nil && walk(node.Leaf.Key, node.Leaf.Value) {
		leaves = append(leaves, node.Leaf)
	}

	for _, e := range node.Edges {
		leaves = append(leaves, collectLeaves(e.Node, walk)...)
	}

	return leaves
}

func longestPrefix(k1, k2 []byte) int {
	max := len(k1)
	if l := len(k2); l < max {
		max = l
	}

	var i int
	for i = 0; i < max; i++ {
		if k1[i] != k2[i] {
			break
		}
	}
	return i
}
