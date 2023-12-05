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

const (
	testIdentName = "testIdent"
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

func TestWithRefLookup(t *testing.T) {
	t.Run("valid", func(t *testing.T) {
		pass := NewSerPass(token.NewFileSet(), WithRefCounter())
		astNode := &ast.Ident{Name: testIdentName}
		ident := SerializeIdent(pass, astNode)
		pass.refCache[astNode] = NewWeakRef(ident)

		// Panic at this point means that inside the function there was a call
		// to the function to the serializer, which we passed as nil.
		//Calling this function means that the link could not be found in the cache,
		// although it should have been there.
		assert.NotPanics(t, func() {
			WithRefLookup[*ast.Ident, *Ident](pass, astNode, nil)
		}, "unexpected: should not try to call serialize func")

		assert.Equal(t, ident, WithRefLookup[*ast.Ident, *Ident](pass, astNode, nil))
	})

	t.Run("invalid: searching for a link that is not in the cache", func(t *testing.T) {
		pass := NewSerPass(token.NewFileSet(), WithRefCounter())
		astNode := &ast.Ident{Name: testIdentName}

		// The absence of panic in this case means that an unknown link was found
		// that for some reason corresponds to the one you were looking for,
		// although you did not save it before
		assert.Panics(t, func() {
			WithRefLookup[*ast.Ident, *Ident](pass, astNode, nil)
		}, "unexpected: should call serialize func")
	})
}

func TestSerProcessFileSize(t *testing.T) {
	t.Run("valid", func(t *testing.T) {
		f, fset := uploadTestData(t, pathToAsonTestDataDir)
		pass := NewSerPass(fset)
		size := calcFileSize(pass, f)
		assert.Greater(t, size, 0)
	})

	t.Run("invalid: with file read error", func(t *testing.T) {
		f, fset := uploadTestData(t, pathToAsonTestDataDir)
		pass := NewSerPass(fset, WithReadFileFn(mockReadFileWithErr))
		size := calcFileSize(pass, f)
		assert.Equal(t, 1<<_GOARCH()-2, size)
	})
}

func TestSerPos(t *testing.T) {
	t.Run("valid: without compression", func(t *testing.T) {
		f, fset := uploadTestData(t, pathToAsonTestDataDir)
		pass := NewSerPass(fset)

		pos := SerializePos(pass, f.FileStart)
		assert.NotNil(t, pos)
		assert.IsType(t, &Position{}, pos)
	})

	t.Run("valid: with compression", func(t *testing.T) {
		f, fset := uploadTestData(t, pathToAsonTestDataDir)
		pass := NewSerPass(fset, WithPosCompression())

		pos := SerializePos(pass, f.FileStart)
		assert.NotNil(t, pos)
		assert.IsType(t, &PosCompressed{}, pos)
	})

	t.Run("invalid: got token.NoPos", func(t *testing.T) {
		pass := NewSerPass(token.NewFileSet())

		pos := SerializePos(pass, token.NoPos)
		assert.NotNil(t, pos)
		assert.Equal(t, pos, new(NoPos))
	})
}

func TestSerializeIdent(t *testing.T) {
	t.Run("valid", func(t *testing.T) {
		astIdent := &ast.Ident{
			Name:    testIdentName,
			NamePos: token.Pos(1),
		}
		pass := NewSerPass(token.NewFileSet())
		ident := SerializeIdent(pass, astIdent)
		assert.NotNil(t, ident)
		assert.Equal(t, testIdentName, ident.Name)
		assert.Equal(t, NodeTypeIdent, ident.Node.Type)
	})
}

func TestSerializeBasicLit(t *testing.T) {

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
