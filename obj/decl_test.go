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
	testFuncDeclName = "DeclFunc"
)

const (
	testFuncDeclWithTypeParams = `package decl
	func DeclFunc[T1, T2 api.Router](a int, b, c string) (any, error) {
		a := api.NewRequest()
		return nil, nil
	}
	`

	testEmptyFuncDecl = `package decl
	func DeclFunc() {}
	`
)

func TestNewFuncDeclObj(t *testing.T) {
	t.Run("valid: empty func decl", func(t *testing.T) {
		funcDeclObj := createFuncDeclObj(t, testEmptyFuncDecl)
		assert.NotNil(t, funcDeclObj)

		assert.Nil(t, funcDeclObj.Recv)
		assert.Equal(t, testFuncDeclName, funcDeclObj.Name.Name)
		assert.True(t, funcDeclObj.Name.IsExported())
		assert.IsType(t, new(FuncTypeObj), funcDeclObj.Type)
		assert.False(t, funcDeclObj.Recursive)

		assert.Nil(t, funcDeclObj.Type.Params)
		assert.Nil(t, funcDeclObj.Type.TypeParams)
		assert.Nil(t, funcDeclObj.Type.DependsParams)
	})

	t.Run("valid: general func decl", func(t *testing.T) {
		funcDeclObj := createFuncDeclObj(t, testFuncDeclWithTypeParams)
		assert.NotNil(t, funcDeclObj)

		assert.Nil(t, funcDeclObj.Recv)
		assert.Equal(t, testFuncDeclName, funcDeclObj.Name.Name)
		assert.True(t, funcDeclObj.Name.IsExported())
		assert.IsType(t, new(FuncTypeObj), funcDeclObj.Type)
		assert.False(t, funcDeclObj.Recursive)

		t.Run("valid: checking the type of a func decl", func(t *testing.T) {
			assert.NotNil(t, funcDeclObj.Type.TypeParams)

			assert.Equal(t, 1, len(funcDeclObj.Type.TypeParams.List))
			assert.Equal(t, 2, funcDeclObj.Type.TypeParams.Len())
			assert.Equal(t, 2, len(funcDeclObj.Type.TypeParams.List[0].Names))
			assert.Equal(t, "T1", funcDeclObj.Type.TypeParams.List[0].Names[0].Name)
			assert.Equal(t, "T2", funcDeclObj.Type.TypeParams.List[0].Names[1].Name)
			assert.Equal(t, "Router", funcDeclObj.Type.TypeParams.List[0].Type)

			assert.Equal(t, 2, len(funcDeclObj.Type.Params.List))
			assert.Equal(t, 3, funcDeclObj.Type.Params.Len())
			assert.Equal(t, 2, len(funcDeclObj.Type.ResultParams.List))
			assert.Equal(t, 2, funcDeclObj.Type.ResultParams.Len())

			assert.NotNil(t, funcDeclObj.Type.DependsParams)
			assert.Equal(t, 1, len(funcDeclObj.Type.DependsParams.List))
			assert.Equal(t, 1, len(funcDeclObj.Type.DependsParams.List[0].Names))
			assert.Equal(t, "api", funcDeclObj.Type.DependsParams.List[0].Names[0].Name)
			assert.Equal(t, "Router", funcDeclObj.Type.DependsParams.List[0].Type)
		})

		t.Run("valid: checking the body of a func decl", func(t *testing.T) {
			assert.NotNil(t, funcDeclObj.Body)

			assert.Nil(t, funcDeclObj.Body.FieldAccess)
			assert.NotNil(t, funcDeclObj.Body.Stmt)
			assert.NotNil(t, funcDeclObj.Body.Stmt.DependsParams)
			assert.Equal(t, 1, len(funcDeclObj.Body.Stmt.DependsParams.List))
			assert.Equal(t, 1, funcDeclObj.Body.Stmt.DependsParams.Len())
			assert.Equal(t, 1, len(funcDeclObj.Body.Stmt.DependsParams.List[0].Names))
			assert.Equal(t, "api", funcDeclObj.Body.Stmt.DependsParams.List[0].Names[0].Name)
			assert.Equal(t, "NewRequest", funcDeclObj.Body.Stmt.DependsParams.List[0].Type)
		})

	})
}

func createFuncDeclObj(t *testing.T, source string) *FuncDeclObj {
	t.Helper()

	fset := token.NewFileSet()
	fobj := NewFileObj(fset, testModuleName, testFileName)
	fobj.Imports = &importTree{Cache: testImportMeta}
	funcDecl := createFuncDecl(t, fset, source)
	funcDeclObj, err := NewFuncDeclObj(fobj, funcDecl)
	assert.NoError(t, err)

	return funcDeclObj
}

func createFuncDecl(t *testing.T, fset *token.FileSet, source string) *ast.FuncDecl {
	t.Helper()

	file, err := parser.ParseFile(fset, "", strings.NewReader(source), parser.AllErrors)
	require.NoError(t, err)
	require.NotNil(t, file)

	var funcDecl *ast.FuncDecl

	for _, decl := range file.Decls {
		decl, ok := decl.(*ast.FuncDecl)
		if !ok {
			continue
		}

		if decl.Name.Name == testFuncDeclName {
			funcDecl = decl
		}
	}

	if funcDecl != nil {
		return funcDecl
	}

	t.Fatalf("failed: cant find %s in source code", testFuncDeclName)
	return nil
}
