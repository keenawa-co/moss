package obj

import (
	"go/ast"
	"go/parser"
	"go/token"
	"strings"
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

const (
	testImportSpec = `package imp
	import "github.com/stretchr/testify/assert"`

	testImportSpecWithName = `package imp
	import testifyAssert "github.com/stretchr/testify/assert"`
)

func TestNewImportObj(t *testing.T) {
	t.Run("valid: create internal import without name (alias)", func(t *testing.T) {
		importSpec := createImportSpec(t, testImportSpec)
		assert.NotNil(t, importSpec)

		importObj := NewImportObj(importSpec, Internal)
		assert.NotNil(t, importObj)

		assert.Equal(t, "\"github.com/stretchr/testify/assert\"", importObj.Path)
		assert.Equal(t, Internal, importObj.ImportKind)
		assert.Nil(t, importObj.Name)
	})

	t.Run("valid: create internal import with name (alias)", func(t *testing.T) {
		importSpec := createImportSpec(t, testImportSpecWithName)
		assert.NotNil(t, importSpec)

		importObj := NewImportObj(importSpec, Internal)
		assert.NotNil(t, importObj)

		assert.Equal(t, "\"github.com/stretchr/testify/assert\"", importObj.Path)
		assert.Equal(t, Internal, importObj.ImportKind)
		assert.NotNil(t, importObj.Name)
		assert.Equal(t, "testifyAssert", importObj.Name.Name)
		assert.Equal(t, Imp, importObj.Name.Kind)
	})
}

func createImportSpec(t *testing.T, source string) *ast.ImportSpec {
	t.Helper()

	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "", strings.NewReader(source), parser.AllErrors)
	require.NoError(t, err)
	require.NotNil(t, file)

	return file.Imports[0]
}
