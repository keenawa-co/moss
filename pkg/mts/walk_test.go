package mts

import (
	"reflect"
	"testing"

	"github.com/stretchr/testify/assert"
)

type mockHandler struct {
	visitedFields []*Field
}

func (m *mockHandler) Handle(field *Field) {
	m.visitedFields = append(m.visitedFields, field)
}

func TestWalk(t *testing.T) {
	t.Run("Walk through a simple map", func(t *testing.T) {
		data := map[string]int{"one": 1, "two": 2}
		handler := &mockHandler{}
		Walk(handler, &Field{
			Value: data,
			Path:  []string{},
			Kind:  reflect.TypeOf(data).Kind(),
		})

		expectedFields := len(data) + 1 // add 1 for the root element
		assert.Equal(t, expectedFields, len(handler.visitedFields), "Expected to visit all fields plus the root")

		rootVisited := false
		for _, field := range handler.visitedFields {
			if len(field.Path) == 0 {
				rootVisited = true
				assert.Equal(t, reflect.Map, field.Kind, "Kind of root should be map")
			}
		}
		assert.True(t, rootVisited, "The root element should be visited")
	})

	t.Run("Walk through a simple map", func(t *testing.T) {
		data := map[string]int{"one": 1, "two": 2}
		handler := &mockHandler{}

		Walk(handler, &Field{
			Value: data,
			Path:  []string{},
			Kind:  reflect.TypeOf(data).Kind(),
		})

		assert.Equal(t, 3, len(handler.visitedFields), "Expected to visit 3 fields (1 root + 2 map entries)")

		// checking the correct paths and types for each field
		rootVisited := false
		for _, field := range handler.visitedFields {
			if len(field.Path) == 0 {
				rootVisited = true
				assert.Equal(t, reflect.Map, field.Kind, "Root kind should be map")
			} else {
				assert.Contains(t, []string{"one", "two"}, field.Path[len(field.Path)-1], "Path should contain the key")
				assert.Equal(t, reflect.Int, field.Kind, "Kind should be int for map entries")
			}
		}
		assert.True(t, rootVisited, "The root element should be visited")
	})

	t.Run("Walk through a nested structure", func(t *testing.T) {
		data := map[string]interface{}{
			"numbers": []int{1, 2, 3},
			"nested": map[string]string{
				"hello": "world",
			},
		}
		handler := &mockHandler{}

		Walk(handler, &Field{
			Value: data,
			Path:  []string{},
		})

		assert.Greater(t, len(handler.visitedFields), 2, "Should visit more than 2 fields in a nested structure")

		// verifying that specific fields have been visited
		visitedNumbers, visitedHello := checkNestedFieldsVisited(handler.visitedFields)
		assert.True(t, visitedNumbers, "Should visit the 'numbers' slice")
		assert.True(t, visitedHello, "Should visit the 'hello' field in nested map")
	})
}

// checkNestedFieldsVisited checks whether certain fields in a nested structure have been visited.
func checkNestedFieldsVisited(fields []*Field) (visitedNumbers bool, visitedHello bool) {
	for _, field := range fields {
		if len(field.Path) > 1 {
			secondLast := field.Path[len(field.Path)-2]
			last := field.Path[len(field.Path)-1]

			if secondLast == "numbers" && last[0] == '[' {
				visitedNumbers = true
			}
			if secondLast == "nested" && last == "hello" {
				visitedHello = true
			}
		}
	}
	return
}
