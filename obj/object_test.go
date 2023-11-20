package obj

import (
	"go/ast"
	"testing"

	"github.com/stretchr/testify/assert"
)

const (
	testModuleName = "github.com/4rchr4y/testcode"
	testFileName   = "test_name.go"
)

const (
	testIdentName = "testIdent"
)

var (
	testImportMeta = map[string]int{
		"api":   0,
		"db":    1,
		"event": 2,
	}
)

func TestNewIdentObj(t *testing.T) {
	t.Run("valid: correct creation of IdentObj from *ast.Ident", func(t *testing.T) {
		ident := NewIdentObj(&ast.Ident{Name: testIdentName})

		assert.Equal(t, testIdentName, ident.Name)
	})

	t.Run("invalid: correct creation of IdentObj from *ast.Ident", func(t *testing.T) {
		ident := NewIdentObj(&ast.Ident{})

		assert.Empty(t, ident.Name)
	})
}

func TestIdentObjIsExported(t *testing.T) {
	t.Run("valid: correct behavior for public names", func(t *testing.T) {
		ident := &ast.Ident{Name: "Public"}
		assert.True(t, ident.IsExported())
	})

	t.Run("valid: correct behavior for private names", func(t *testing.T) {
		ident := &ast.Ident{Name: "private"}
		assert.False(t, ident.IsExported())
	})
}

func TestIdentObjString(t *testing.T) {
	t.Run("valid: correct creation of Ident object", func(t *testing.T) {
		ident := &IdentObj{
			Name: testIdentName,
			Kind: Var,
		}

		assert.Equal(t, testIdentName, ident.String())
		assert.NotEqual(t, "<nil>", ident.String())
	})

	t.Run("invalid: creation Ident object without name", func(t *testing.T) {
		var ident *IdentObj

		assert.Equal(t, "<nil>", ident.String())
		assert.NotEqual(t, testIdentName, ident.String())
	})
}

func TestFieldObjListLen(t *testing.T) {
	list := &FieldObjList{
		List: []*FieldObj{
			{Names: []*IdentObj{{Name: "a1"}}},
			{Names: []*IdentObj{{Name: "a2"}, {Name: "a3"}}},
			{Names: []*IdentObj{{Name: "a4"}}},
			{Names: []*IdentObj{{Name: "a5"}}},
			{Names: []*IdentObj{{Name: "a6"}, {Name: "a7"}, {Name: "a7"}, {Name: "a7"}, {Name: "a7"}}},
		},
	}

	t.Run("valid: correct length of list items", func(t *testing.T) {
		assert.Equal(t, 10, list.Len())
	})

	t.Run("invalid: not correct length of list items", func(t *testing.T) {
		assert.NotEqual(t, 100, list.Len())
	})
}
