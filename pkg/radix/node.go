package radix

import (
	"fmt"
	"sort"
)

type Leaf[T any] struct {
	Key   []byte
	Value T
}

type Edge[T any] struct {
	Label byte
	Node  *Node[T]
}

type EdgeList[T any] []*Edge[T]

func (e EdgeList[T]) Len() int {
	return len(e)
}

func (e EdgeList[T]) Less(i, j int) bool {
	return e[i].Label < e[j].Label
}

func (e EdgeList[T]) Swap(i, j int) {
	e[i], e[j] = e[j], e[i]
}

func (e EdgeList[T]) Sort() {
	sort.Sort(e)
}

type Node[T any] struct {
	Leaf   *Leaf[T]
	Prefix []byte
	Edges  EdgeList[T]
}

func (n *Node[T]) isLeaf() bool {
	return n.Leaf != nil
}

func (n *Node[T]) addEdge(e *Edge[T]) {
	num := len(n.Edges)
	idx := sort.Search(num, func(i int) bool {
		return n.Edges[i].Label >= e.Label
	})

	// edge to be added goes to the end of the list, simply add it
	if idx == num {
		n.Edges = append(n.Edges, e)
		return
	}

	// expand the slice, insert a new edge and move the rest
	n.Edges = append(n.Edges, nil)       // add space for a new edge
	copy(n.Edges[idx+1:], n.Edges[idx:]) // shift elements to the right
	n.Edges[idx] = e
}

func (n *Node[T]) updateEdge(label byte, node *Node[T]) error {
	num := len(n.Edges)
	idx := sort.Search(num, func(i int) bool {
		return n.Edges[i].Label >= label
	})

	if idx < num && n.Edges[idx].Label == label {
		// check if the node is already a node of this edge.
		if n.Edges[idx].Node != node {
			n.Edges[idx].Node = node
		}
		return nil
	}

	panic(fmt.Sprintf("updating missing edge with label: %d", label))
}

func (n *Node[T]) getEdge(label byte) *Node[T] {
	if len(n.Edges) == 0 {
		return nil
	}

	// For very small lists of edges, linear search can be faster
	// than binary search due to its simplicity and lack of overhead
	if len(n.Edges) < 10 {
		for _, edge := range n.Edges {
			if edge.Label == label {
				return edge.Node
			}
		}
		return nil
	}

	idx := sort.Search(len(n.Edges), func(i int) bool {
		return n.Edges[i].Label >= label
	})

	if idx < len(n.Edges) && n.Edges[idx].Label == label {
		return n.Edges[idx].Node
	}
	return nil
}

func (n *Node[T]) delEdge(label byte) {
	idx := sort.Search(len(n.Edges), func(i int) bool {
		return n.Edges[i].Label >= label
	})

	if idx < len(n.Edges) && n.Edges[idx].Label == label {
		// remove an edge, replacing it with the last element
		// of the list and decreasing the list size by 1
		n.Edges[idx] = n.Edges[len(n.Edges)-1]
		n.Edges = n.Edges[:len(n.Edges)-1]
	}
}

func (n *Node[T]) mergeChild() {
	e := n.Edges[0]
	child := e.Node
	n.Prefix = append(n.Prefix, child.Prefix...)
	n.Leaf = child.Leaf
	n.Edges = child.Edges
}
