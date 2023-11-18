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

var (
	testModuleName = "github.com/4rchr4y/testcode"
	testFileName   = "test_name.go"
	testFuncType   = "TypeFunc"
	testImportMeta = map[string]int{
		"api":   0,
		"db":    1,
		"event": 2,
	}
)

var (
	testFuncTypeCode = `package func_code
	type TypeFunc func(a string, b, c int) (float64, error)`

	testFuncTypeWithDepsCode = `package func_code
	type TypeFunc func(a string, b db.Connect, c, d int) (*api.Request, error)`
)

func TestNewFuncTypeObj(t *testing.T) {
	t.Run("valid: test type func with dependencies ", func(t *testing.T) {
		funcTypeObj := createFuncTypeObj(t, testFuncTypeWithDepsCode)
		assert.NotNil(t, funcTypeObj)

		t.Run("valid: checking (incoming) function params", func(t *testing.T) {
			assert.Equal(t, 3, len(funcTypeObj.Params.List))
			assert.Equal(t, 4, funcTypeObj.Params.Len())

			assert.Equal(t, 1, len(funcTypeObj.Params.List[0].Names))
			assert.Equal(t, "string", funcTypeObj.Params.List[0].Type)
			assert.Equal(t, "a", funcTypeObj.Params.List[0].Names[0].Name)

			assert.Equal(t, 1, len(funcTypeObj.Params.List[1].Names))
			assert.Equal(t, "Connect", funcTypeObj.Params.List[1].Type)
			assert.Equal(t, "b", funcTypeObj.Params.List[1].Names[0].Name)

			assert.Equal(t, 2, len(funcTypeObj.Params.List[2].Names))
			assert.Equal(t, "int", funcTypeObj.Params.List[2].Type)
			assert.Equal(t, "c", funcTypeObj.Params.List[2].Names[0].Name)
			assert.Equal(t, "d", funcTypeObj.Params.List[2].Names[1].Name)
		})

		t.Run("valid: checking (outgoing) function params", func(t *testing.T) {
			assert.Equal(t, 2, len(funcTypeObj.ResultParams.List))
			assert.Equal(t, 2, funcTypeObj.ResultParams.Len())

			assert.Equal(t, 0, len(funcTypeObj.ResultParams.List[0].Names))
			assert.Equal(t, "Request", funcTypeObj.ResultParams.List[0].Type)

			assert.Equal(t, 0, len(funcTypeObj.ResultParams.List[1].Names))
			assert.Equal(t, "error", funcTypeObj.ResultParams.List[1].Type)
		})

		t.Run("valid: checking function dependencies params", func(t *testing.T) {
			assert.Equal(t, 2, len(funcTypeObj.DependsParams.List))
			assert.Equal(t, 2, funcTypeObj.DependsParams.Len())

			assert.Equal(t, 1, len(funcTypeObj.DependsParams.List[0].Names))
			assert.Equal(t, "db", funcTypeObj.DependsParams.List[0].Names[0].Name)
			assert.Equal(t, "Connect", funcTypeObj.DependsParams.List[0].Type)

			assert.Equal(t, 1, len(funcTypeObj.DependsParams.List[1].Names))
			assert.Equal(t, "api", funcTypeObj.DependsParams.List[1].Names[0].Name)
			assert.Equal(t, "Request", funcTypeObj.DependsParams.List[1].Type)
		})

		t.Run("invalid: checking function dependencies params", func(t *testing.T) {
			assert.NotEqual(t, 20, len(funcTypeObj.DependsParams.List))
			assert.NotEqual(t, 20, funcTypeObj.DependsParams.Len())

			assert.NotEqual(t, 10, len(funcTypeObj.DependsParams.List[0].Names))
			assert.NotEqual(t, "db2", funcTypeObj.DependsParams.List[0].Names[0].Name)
			assert.NotEqual(t, "NotConnect", funcTypeObj.DependsParams.List[0].Type)

			assert.NotEqual(t, 10, len(funcTypeObj.DependsParams.List[1].Names))
			assert.NotEqual(t, "api2", funcTypeObj.DependsParams.List[1].Names[0].Name)
			assert.NotEqual(t, "NotRequest", funcTypeObj.DependsParams.List[1].Type)
		})
	})

	t.Run("valid: test type func without dependencies", func(t *testing.T) {
		funcTypeObj := createFuncTypeObj(t, testFuncTypeCode)

		t.Run("valid: checking (incoming) function params", func(t *testing.T) {
			assert.Equal(t, 2, len(funcTypeObj.Params.List))
			assert.Equal(t, 3, funcTypeObj.Params.Len())
			assert.Equal(t, 2, len(funcTypeObj.Params.List[1].Names))

			assert.Equal(t, "a", funcTypeObj.Params.List[0].Names[0].Name)
			assert.Equal(t, "b", funcTypeObj.Params.List[1].Names[0].Name)
			assert.Equal(t, "c", funcTypeObj.Params.List[1].Names[1].Name)

			assert.Equal(t, "string", funcTypeObj.Params.List[0].Type)
			assert.Equal(t, "int", funcTypeObj.Params.List[1].Type)
		})

		t.Run("valid: checking (outgoing) function params", func(t *testing.T) {
			assert.Equal(t, 2, len(funcTypeObj.ResultParams.List))

			assert.Equal(t, "float64", funcTypeObj.ResultParams.List[0].Type)
			assert.Equal(t, 0, len(funcTypeObj.ResultParams.List[0].Names))

			assert.Equal(t, "error", funcTypeObj.ResultParams.List[1].Type)
			assert.Equal(t, 0, len(funcTypeObj.ResultParams.List[1].Names))
		})

		t.Run("valid: checking fields that should not be initialized", func(t *testing.T) {
			assert.Nil(t, funcTypeObj.TypeParams)
			assert.Nil(t, funcTypeObj.DependsParams)
		})
	})
}

func createFuncTypeObj(t *testing.T, source string) *FuncTypeObj {
	t.Helper()

	fset := token.NewFileSet()
	fobj := NewFileObj(fset, testModuleName, testFileName)
	fobj.Imports = &importTree{Cache: testImportMeta}
	funcType := createFuncType(t, fset, source)
	funcTypeObj, err := NewFuncTypeObj(fobj, funcType)
	assert.NoError(t, err)

	return funcTypeObj
}

func createFuncType(t *testing.T, fset *token.FileSet, source string) *ast.FuncType {
	t.Helper()

	file, err := parser.ParseFile(fset, "", strings.NewReader(source), parser.AllErrors)
	require.NoError(t, err)
	require.NotNil(t, file)

	var funcType *ast.FuncType
	for _, decl := range file.Decls {
		genDecl, ok := decl.(*ast.GenDecl)
		if !ok {
			continue
		}

		for _, spec := range genDecl.Specs {
			typeSpec, ok := spec.(*ast.TypeSpec)
			if !ok {
				continue
			}

			if typeSpec.Name.Name == testFuncType {
				funcType, _ = typeSpec.Type.(*ast.FuncType)
				break
			}
		}
	}

	if funcType != nil {
		return funcType
	}

	t.Fatalf("failed: cant find %s in source code", testFuncType)
	return nil
}
