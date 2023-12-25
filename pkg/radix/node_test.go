package radix

import (
	"fmt"
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

type testType struct {
	Value int
}

func TestLen(t *testing.T) {
	list := EdgeList[testType]{
		{Label: 1, Node: &Node[testType]{}},
		{Label: 2, Node: &Node[testType]{}},
	}

	assert.Equal(t, 2, list.Len(), "Len should return the correct length")
	emptyList := EdgeList[testType]{}
	assert.Equal(t, 0, emptyList.Len(), "Len should return 0 for empty list")
}

func TestLess(t *testing.T) {
	list := EdgeList[testType]{
		{Label: 1, Node: &Node[testType]{}},
		{Label: 2, Node: &Node[testType]{}},
	}

	assert.True(t, list.Less(0, 1), "Less should return true for label 1 < label 2")
	assert.False(t, list.Less(1, 0), "Less should return false for label 2 > label 1")
}

func TestSwap(t *testing.T) {
	list := EdgeList[testType]{
		{Label: 1, Node: &Node[testType]{}},
		{Label: 2, Node: &Node[testType]{}},
	}

	first, second := list[0], list[1]
	list.Swap(0, 1)

	assert.Equal(t, second, list[0], "Swap should move the second element to the first position")
	assert.Equal(t, first, list[1], "Swap should move the first element to the second position")
}

func TestSort(t *testing.T) {
	list := EdgeList[testType]{
		{Label: 3, Node: &Node[testType]{}},
		{Label: 1, Node: &Node[testType]{}},
		{Label: 2, Node: &Node[testType]{}},
	}

	list.Sort()

	assert.True(t, sort.IsSorted(list), "Sort should sort the list in ascending order based on Label")

	emptyList := EdgeList[testType]{}
	emptyList.Sort()
	assert.True(t, sort.IsSorted(emptyList), "Sort should handle an empty list without error")
}

func TestIsLeaf(t *testing.T) {
	t.Run("Node with a leaf", func(t *testing.T) {
		leaf := &Leaf[string]{Key: []byte("key"), Value: "value"}
		node := Node[string]{Leaf: leaf}
		assert.True(t, node.isLeaf(), "isLeaf should return true when the node has a leaf")
	})

	t.Run("Node without a leaf", func(t *testing.T) {
		node := Node[string]{}
		assert.False(t, node.isLeaf(), "isLeaf should return false when the node does not have a leaf")
	})

	t.Run("Node with nil leaf", func(t *testing.T) {
		node := Node[string]{Leaf: nil}
		assert.False(t, node.isLeaf(), "isLeaf should return false when the node's leaf is nil")
	})
}

func TestAddEdge(t *testing.T) {
	t.Run("Add edge to empty node", func(t *testing.T) {
		node := Node[string]{}
		edge := &Edge[string]{Label: 1, Node: &Node[string]{}}
		node.addEdge(edge)
		assert.Equal(t, 1, len(node.Edges), "The node should have 1 edge after adding")
		assert.Equal(t, edge, node.Edges[0], "The first edge of the node should be the added edge")
	})

	t.Run("Add edge to node with existing edges", func(t *testing.T) {
		node := Node[string]{
			Edges: []*Edge[string]{{Label: 1, Node: &Node[string]{}}},
		}
		newEdge := &Edge[string]{Label: 2, Node: &Node[string]{}}
		node.addEdge(newEdge)
		assert.Equal(t, 2, len(node.Edges), "The node should have 2 edges after adding")
		assert.Equal(t, newEdge, node.Edges[1], "The second edge of the node should be the new edge")
	})

	t.Run("Add edge in the middle of existing edges", func(t *testing.T) {
		node := Node[string]{
			Edges: []*Edge[string]{{Label: 1, Node: &Node[string]{}}, {Label: 3, Node: &Node[string]{}}},
		}
		middleEdge := &Edge[string]{Label: 2, Node: &Node[string]{}}
		node.addEdge(middleEdge)
		assert.Equal(t, 3, len(node.Edges), "The node should have 3 edges after adding")
		assert.Equal(t, middleEdge, node.Edges[1], "The new edge should be positioned in the middle")
	})

	t.Run("Add edge with the same label", func(t *testing.T) {
		node := Node[string]{
			Edges: []*Edge[string]{{Label: 1, Node: &Node[string]{}}, {Label: 3, Node: &Node[string]{}}},
		}
		sameLabelEdge := &Edge[string]{Label: 1, Node: &Node[string]{}}
		node.addEdge(sameLabelEdge)
		assert.Equal(t, 3, len(node.Edges), "The node should have 3 edges after adding")
		assert.Equal(t, sameLabelEdge, node.Edges[0], "The new edge with the same label should replace the old one")
	})

	t.Run("Add multiple edges and check order", func(t *testing.T) {
		node := Node[string]{}
		edges := []*Edge[string]{
			{Label: 5, Node: &Node[string]{}},
			{Label: 3, Node: &Node[string]{}},
			{Label: 4, Node: &Node[string]{}},
			{Label: 1, Node: &Node[string]{}},
			{Label: 2, Node: &Node[string]{}},
		}
		for _, e := range edges {
			node.addEdge(e)
		}
		assert.Equal(t, 5, len(node.Edges), "The node should have 5 edges after adding")
		for i := 0; i < len(node.Edges)-1; i++ {
			assert.True(t, node.Edges[i].Label < node.Edges[i+1].Label, "The edges should be sorted in ascending order")
		}
	})

	t.Run("Add edge with lower label to the end", func(t *testing.T) {
		node := Node[string]{
			Edges: []*Edge[string]{{Label: 2, Node: &Node[string]{}}},
		}
		lowLabelEdge := &Edge[string]{Label: 1, Node: &Node[string]{}}
		node.addEdge(lowLabelEdge)
		assert.Equal(t, 2, len(node.Edges), "The node should have 2 edges after adding")
		assert.Equal(t, lowLabelEdge, node.Edges[0], "The new edge with a lower label should be at the beginning")
	})

	t.Run("Add edge with higher label to the beginning", func(t *testing.T) {
		node := Node[string]{
			Edges: []*Edge[string]{{Label: 1, Node: &Node[string]{}}},
		}
		highLabelEdge := &Edge[string]{Label: 2, Node: &Node[string]{}}
		node.addEdge(highLabelEdge)
		assert.Equal(t, 2, len(node.Edges), "The node should have 2 edges after adding")
		assert.Equal(t, highLabelEdge, node.Edges[1], "The new edge with a higher label should be at the end")
	})

	t.Run("Add edge to node with many edges", func(t *testing.T) {
		node := Node[string]{}
		for i := 0; i < 1000; i++ {
			node.Edges = append(node.Edges, &Edge[string]{Label: byte(i), Node: &Node[string]{}})
		}
		middleEdge := &Edge[string]{Label: 0, Node: &Node[string]{}}
		node.addEdge(middleEdge)
		assert.Equal(t, 1001, len(node.Edges), "The node should have 1001 edges after adding")

		// check that the new edge is in the correct position.
		found := false
		for _, e := range node.Edges {
			if e == middleEdge {
				found = true
				break
			}
		}
		assert.True(t, found, "The new edge should be in the node's edges")
	})

}

func TestUpdateEdge(t *testing.T) {
	t.Run("Update existing edge", func(t *testing.T) {
		node := Node[string]{
			Edges: EdgeList[string]{
				{Label: 1, Node: &Node[string]{Leaf: &Leaf[string]{Value: "old"}}},
			},
		}

		newNode := &Node[string]{Leaf: &Leaf[string]{Value: "new"}}
		node.updateEdge(1, newNode)

		assert.Equal(t, newNode, node.Edges[0].Node, "The node for the existing edge should be updated")
		assert.Equal(t, "new", node.Edges[0].Node.Leaf.Value, "The value of the updated node should be 'new'")
	})

	t.Run("Update edge in a list of edges", func(t *testing.T) {
		node := Node[string]{
			Edges: EdgeList[string]{
				{Label: 1, Node: &Node[string]{}},
				{Label: 2, Node: &Node[string]{}},
				{Label: 3, Node: &Node[string]{}},
			},
		}

		newNode := &Node[string]{Leaf: &Leaf[string]{Value: "new"}}
		node.updateEdge(2, newNode)

		assert.Equal(t, newNode, node.Edges[1].Node, "The node for the existing edge with label 2 should be updated")
	})

	t.Run("Attempt to update non-existing edge", func(t *testing.T) {
		node := Node[string]{
			Edges: EdgeList[string]{{Label: 1, Node: &Node[string]{}}},
		}

		expectedPanicMessage := fmt.Sprintf("updating missing edge with label: %d", 2)
		assert.PanicsWithValue(t, expectedPanicMessage, func() { node.updateEdge(2, &Node[string]{}) },
			"Attempting to update a non-existing edge should cause a panic with the appropriate message")
	})

	t.Run("Update edge with same label but different node", func(t *testing.T) {
		node := Node[string]{
			Edges: EdgeList[string]{
				{Label: 1, Node: &Node[string]{Leaf: &Leaf[string]{Value: "old1"}}},
				{Label: 2, Node: &Node[string]{Leaf: &Leaf[string]{Value: "old2"}}},
			},
		}

		newNode := &Node[string]{Leaf: &Leaf[string]{Value: "new"}}
		node.updateEdge(1, newNode)

		assert.Equal(t, newNode, node.Edges[0].Node, "The node for the existing edge with label 1 should be updated")
		assert.Equal(t, "new", node.Edges[0].Node.Leaf.Value, "The value of the updated node should be 'new'")
	})
}

func TestGetEdge(t *testing.T) {
	t.Run("Get existing edge", func(t *testing.T) {
		expectedNode := &Node[string]{Leaf: &Leaf[string]{Value: "value"}}
		node := Node[string]{
			Edges: EdgeList[string]{
				{Label: 1, Node: &Node[string]{}},
				{Label: 2, Node: expectedNode},
				{Label: 3, Node: &Node[string]{}},
			},
		}

		result := node.getEdge(2)
		assert.Equal(t, expectedNode, result, "getEdge should return the correct node for an existing edge")
	})

	t.Run("Get non-existing edge", func(t *testing.T) {
		node := Node[string]{
			Edges: EdgeList[string]{
				{Label: 1, Node: &Node[string]{}},
				{Label: 3, Node: &Node[string]{}},
			},
		}

		result := node.getEdge(2)
		assert.Nil(t, result, "getEdge should return nil for a non-existing edge")
	})

	t.Run("Get edge from empty edge list", func(t *testing.T) {
		node := Node[string]{}

		result := node.getEdge(1)
		assert.Nil(t, result, "getEdge should return nil when the edge list is empty")
	})

	t.Run("Get edge with the lowest label", func(t *testing.T) {
		lowestNode := &Node[string]{Leaf: &Leaf[string]{Value: "lowest"}}
		node := Node[string]{
			Edges: EdgeList[string]{
				{Label: 1, Node: lowestNode},
				{Label: 2, Node: &Node[string]{}},
				{Label: 3, Node: &Node[string]{}},
			},
		}

		result := node.getEdge(1)
		assert.Equal(t, lowestNode, result, "getEdge should correctly return the node with the lowest label")
	})

	t.Run("Get edge with the highest label", func(t *testing.T) {
		highestNode := &Node[string]{Leaf: &Leaf[string]{Value: "highest"}}
		node := Node[string]{
			Edges: EdgeList[string]{
				{Label: 1, Node: &Node[string]{}},
				{Label: 2, Node: &Node[string]{}},
				{Label: 3, Node: highestNode},
			},
		}

		result := node.getEdge(3)
		assert.Equal(t, highestNode, result, "getEdge should correctly return the node with the highest label")
	})
}

func TestDelEdge(t *testing.T) {
	t.Run("Delete existing edge", func(t *testing.T) {
		node := Node[string]{
			Edges: EdgeList[string]{
				{Label: 1, Node: &Node[string]{}},
				{Label: 2, Node: &Node[string]{}},
				{Label: 3, Node: &Node[string]{}},
			},
		}

		node.delEdge(2)
		assert.Equal(t, 2, len(node.Edges), "The node should have 2 edges after deletion")
		assert.Equal(t, byte(1), node.Edges[0].Label, "The first edge should have label 1")
		assert.Equal(t, byte(3), node.Edges[1].Label, "The second edge should have label 3")
	})

	t.Run("Delete non-existing edge", func(t *testing.T) {
		node := Node[string]{
			Edges: EdgeList[string]{
				{Label: 1, Node: &Node[string]{}},
				{Label: 3, Node: &Node[string]{}},
			},
		}

		node.delEdge(2)
		assert.Equal(t, 2, len(node.Edges), "The node should still have 2 edges after attempting to delete a non-existing edge")
	})

	t.Run("Delete edge from empty edge list", func(t *testing.T) {
		node := Node[string]{}

		node.delEdge(1)
		assert.Equal(t, 0, len(node.Edges), "The node should still have 0 edges after attempting to delete an edge from an empty list")
	})

	t.Run("Delete the only edge", func(t *testing.T) {
		node := Node[string]{
			Edges: EdgeList[string]{{Label: 1, Node: &Node[string]{}}},
		}

		node.delEdge(1)
		assert.Equal(t, 0, len(node.Edges), "The node should have 0 edges after deleting the only edge")
	})

	t.Run("Delete the first edge", func(t *testing.T) {
		node := Node[string]{
			Edges: EdgeList[string]{
				{Label: 1, Node: &Node[string]{}},
				{Label: 2, Node: &Node[string]{}},
			},
		}

		node.delEdge(1)
		assert.Equal(t, 1, len(node.Edges), "The node should have 1 edge after deleting the first edge")
		assert.Equal(t, byte(2), node.Edges[0].Label, "The remaining edge should have label 2")
	})

	t.Run("Delete the last edge", func(t *testing.T) {
		node := Node[string]{
			Edges: EdgeList[string]{
				{Label: 1, Node: &Node[string]{}},
				{Label: 2, Node: &Node[string]{}},
			},
		}

		node.delEdge(2)
		assert.Equal(t, 1, len(node.Edges), "The node should have 1 edge after deleting the last edge")
		assert.Equal(t, byte(1), node.Edges[0].Label, "The remaining edge should have label 1")
	})
}
