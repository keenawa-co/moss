package ason

import (
	"errors"
	"go/ast"
	"go/parser"
	"go/token"
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

const (
	pathToAsonTestDataDir = "./testdata/main.go"
)

func mockReadFileWithErr(name string) ([]byte, error) {
	return nil, errors.New("test error")
}

func TestNewSerPass(t *testing.T) {
	t.Run("valid: creation of pass without any params", func(t *testing.T) {
		fset := token.NewFileSet()
		pass := NewSerPass(fset)

		assert.NotNil(t, pass)
		assert.NotNil(t, pass.fset)
		assert.Nil(t, pass.refCache)
	})

	t.Run("valid: creation of pass with RefCounterEnable", func(t *testing.T) {
		fset := token.NewFileSet()
		pass := NewSerPass(fset, WithRefCounter())

		assert.NotNil(t, pass)
		assert.NotNil(t, pass.fset)
		assert.NotNil(t, pass.refCache)
		assert.Equal(t, true, pass.conf.RefCounterEnable)
	})

	t.Run("valid: creation of pass with PosCompress", func(t *testing.T) {
		fset := token.NewFileSet()
		pass := NewSerPass(fset, WithPosCompression())

		assert.NotNil(t, pass)
		assert.NotNil(t, pass.fset)
		assert.Nil(t, pass.refCache)
		assert.Equal(t, true, pass.conf.PosCompress)
	})

	t.Run("valid: creation of pass with params", func(t *testing.T) {
		fset := token.NewFileSet()
		pass := NewSerPass(fset, WithPosCompression(), WithRefCounter())

		assert.NotNil(t, pass)
		assert.NotNil(t, pass.fset)
		assert.NotNil(t, pass.refCache)
		assert.Equal(t, true, pass.conf.PosCompress)
		assert.Equal(t, true, pass.conf.RefCounterEnable)
	})
}

func TestSerializeNode(t *testing.T) {
	t.Run("valid: correct creation", func(t *testing.T) {
		node := SerializeNode(&ast.BasicLit{})
		assert.Equal(t, node.Type, NodeTypeBasicLit)
	})

	t.Run("invalid: correct creation but expect wrong type", func(t *testing.T) {
		node := SerializeNode(&ast.BasicLit{})
		assert.NotEqual(t, node.Type, NodeTypeComment)
	})
}

func TestSerializeNodeWithRef(t *testing.T) {
	t.Run("valid: correct creation", func(t *testing.T) {
		node := SerializeNodeWithRef(&ast.BasicLit{}, uint(1))
		assert.Equal(t, node.Type, NodeTypeBasicLit)
		assert.Equal(t, node.Ref, uint(1))
	})

	t.Run("invalid: correct creation but expect wrong type", func(t *testing.T) {
		node := SerializeNodeWithRef(&ast.BasicLit{}, uint(1))
		assert.NotEqual(t, node.Type, NodeTypeComment)
		assert.NotEqual(t, node.Ref, uint(2))
	})
}

func TestSerProcessFileSize(t *testing.T) {
	t.Run("valid", func(t *testing.T) {
		f, fset := uploadTestData(t, pathToAsonTestDataDir)
		pass := NewSerPass(fset)
		size := SerProcessFileSize(pass, f)
		assert.Greater(t, size, 0)
	})

	t.Run("invalid: with file read error", func(t *testing.T) {
		f, fset := uploadTestData(t, pathToAsonTestDataDir)
		pass := NewSerPass(fset, WithReadFileFn(mockReadFileWithErr))
		size := SerProcessFileSize(pass, f)
		assert.Equal(t, 1<<_GOARCH()-2, size)
	})
}

func uploadTestData(t *testing.T, pathToTestData string) (*ast.File, *token.FileSet) {
	t.Helper()

	fset := token.NewFileSet()
	require.NotNil(t, fset)

	f, err := parser.ParseFile(fset, pathToTestData, nil, parser.AllErrors)
	if err != nil {
		t.Fatal(err)
	}
	require.NotNil(t, f)

	return f, fset
}
