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
	testIdentType = "testType"
)

var (
	testValueSpecs = []*ast.ValueSpec{
		{
			Names: []*ast.Ident{{Name: testIdentName}},
			Type:  &ast.Ident{Name: testIdentType},
			Values: []ast.Expr{
				&ast.BinaryExpr{
					X: &ast.Ident{Name: "iota"},
					Y: &ast.BasicLit{Kind: token.INT, Value: "2"},
				},
			},
		},
		{
			Names:  []*ast.Ident{{Name: testIdentName}},
			Type:   nil,
			Values: nil,
		},
	}
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

	t.Run("valid: creation of pass with CACHE_REF", func(t *testing.T) {
		fset := token.NewFileSet()
		pass := NewSerPass(fset, WithSerConf(CACHE_REF))

		assert.NotNil(t, pass)
		assert.NotNil(t, pass.fset)
		assert.NotNil(t, pass.refCache)
		assert.NotNil(t, true, pass.conf[CACHE_REF])
	})

	t.Run("valid: use tailNode", func(t *testing.T) {

		pass := NewSerPass(token.NewFileSet())
		serSpecs := make([]*ValueSpec, len(testValueSpecs))
		for i, s := range testValueSpecs {
			serSpecs[i] = SerializeValueSpec(pass, s)
			// tail saving
			pass.tailNode = serSpecs[i]
		}

		assert.NotNil(t, serSpecs[1].Type)
		assert.Equal(t, serSpecs[1].Type.(*Ident).Name, testIdentType)
		assert.Equal(t, "iota", serSpecs[1].Values[0].(*BinaryExpr).X.(*Ident).Name)
		assert.Equal(t, token.INT.String(), serSpecs[1].Values[0].(*BinaryExpr).Y.(*BasicLit).Kind)
		assert.Equal(t, "2", serSpecs[1].Values[0].(*BinaryExpr).Y.(*BasicLit).Value)
	})

	t.Run("invalid: not saving the tail", func(t *testing.T) {
		pass := NewSerPass(token.NewFileSet())
		serSpecs := make([]*ValueSpec, len(testValueSpecs))
		for i, s := range testValueSpecs {
			serSpecs[i] = SerializeValueSpec(pass, s)
		}

		assert.Nil(t, serSpecs[1].Type)
		assert.Nil(t, serSpecs[1].Values)
	})
}

func TestWithRefLookup(t *testing.T) {
	t.Run("valid", func(t *testing.T) {
		pass := NewSerPass(token.NewFileSet(), WithSerConf(CACHE_REF))
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
		pass := NewSerPass(token.NewFileSet(), WithSerConf(CACHE_REF))
		astNode := &ast.Ident{Name: testIdentName}

		// The absence of panic in this case means that an unknown link was found
		// that for some reason corresponds to the one you were looking for,
		// although you did not save it before
		assert.Panics(t, func() {
			WithRefLookup[*ast.Ident, *Ident](pass, astNode, nil)
		}, "unexpected: should call serialize func")
	})
}

func TestSerializeOption(t *testing.T) {
	t.Run("valid: with non-nil input", func(t *testing.T) {
		pass := &serPass{}
		input := &ast.Ident{Name: testIdentName}
		result := SerializeOption(pass, input, SerializeIdent)
		assert.NotNil(t, result)
	})

	t.Run("invalid: with nil input", func(t *testing.T) {
		pass := &serPass{}
		result := SerializeOption(pass, nil, SerializeIdent)
		assert.Nil(t, result)
	})

	t.Run("invalid: with typed nil input", func(t *testing.T) {
		pass := &serPass{}
		var ident *ast.Ident
		result := SerializeOption(pass, ident, SerializeIdent)
		assert.Nil(t, result)
	})
}

func TestCalcFileSize(t *testing.T) {
	t.Run("valid: calculation of file size", func(t *testing.T) {
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

func TestSerializePos(t *testing.T) {
	t.Run("valid: correct creation of serPass", func(t *testing.T) {
		f, fset := uploadTestData(t, pathToAsonTestDataDir)
		pass := NewSerPass(fset)
		pos := SerializePos(pass, f.FileStart)
		assert.NotNil(t, pos)
		assert.IsType(t, &Position{}, pos)
	})

	t.Run("invalid: got token.NoPos", func(t *testing.T) {
		pass := NewSerPass(token.NewFileSet())
		pos := SerializePos(pass, token.NoPos)
		assert.NotNil(t, pos)
		assert.Equal(t, pos, new(NoPos))
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
