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

func (t *Tree[T]) Delete(key []byte) (value T, deleted bool) {
	var parent *Node[T]
	var label byte
	n := t.Root

	for {
		if len(key) == 0 {
			if !n.isLeaf() {
				// node is not a leaf, the key was not found
				return value, false
			}

			// delete a leaf
			value = n.Leaf.Value
			n.Leaf = nil
			t.Size--
			deleted = true
			break
		}

		parent = n
		label = key[0]
		n = n.getEdge(label)
		if n == nil {
			// edge for next key byte not found
			return value, false
		}

		if bytes.HasPrefix(key, n.Prefix) {
			key = key[len(n.Prefix):]
		} else {
			// node prefix does not match key
			return value, false
		}
	}

	if deleted {
		// deleting an edge from its parent if the node has no more children
		if parent != nil && len(n.Edges) == 0 {
			parent.delEdge(label)
		}

		// merge nodes if the node has one child left
		if n != t.Root && len(n.Edges) == 1 {
			n.mergeChild()
		}

		// merge a parent node if it is not the root and has one child left
		if parent != nil && parent != t.Root && len(parent.Edges) == 1 && !parent.isLeaf() {
			parent.mergeChild()
		}
	}

	return value, deleted
}

func (t *Tree[T]) DeletePrefix(prefix []byte) int {
	return t.deletePrefix(nil, t.Root, prefix)
}

func (t *Tree[T]) deletePrefix(parent, node *Node[T], prefix []byte) int {
	if len(prefix) == 0 {
		subTreeSize := 0

		recursiveWalk(node, func(s []byte, v T) bool {
			subTreeSize++
			return false
		})
		if node.isLeaf() {
			node.Leaf = nil
		}
		node.Edges = nil

		if parent != nil && parent != t.Root && len(parent.Edges) == 1 && !parent.isLeaf() {
			parent.mergeChild()
		}
		t.Size -= subTreeSize
		return subTreeSize
	}

	label := prefix[0]
	child := node.getEdge(label)
	if child == nil || (!bytes.HasPrefix(child.Prefix, prefix) && !bytes.HasPrefix(prefix, child.Prefix)) {
		return 0
	}

	if len(child.Prefix) > len(prefix) {
		prefix = prefix[len(prefix):]
	} else {
		prefix = prefix[len(child.Prefix):]
	}
	return t.deletePrefix(node, child, prefix)
}

func (n *Node[T]) mergeChild() {
	e := n.Edges[0]
	child := e.Node
	n.Prefix = append(n.Prefix, child.Prefix...)
	n.Leaf = child.Leaf
	n.Edges = child.Edges
}

func (t *Tree[V]) Load(key []byte) (value V, found bool) {
	n := t.Root

	for {
		if len(key) == 0 {
			if n.isLeaf() {
				return n.Leaf.Value, true
			}
			break
		}

		n = n.getEdge(key[0])
		if n == nil {
			break
		}

		if bytes.HasPrefix(key, n.Prefix) {
			key = key[len(n.Prefix):]
		} else {
			break
		}
	}
	return value, found
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

	i := 0
	k := len(node.Edges)
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
