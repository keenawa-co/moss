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
	testModuleName     = "github.com/4rchr4y/testcode"
	testFileName       = "test_name.go"
	testFuncTypeName   = "TypeFunc"
	testStructTypeName = "TypeStruct"
	testImportMeta     = map[string]int{
		"api":   0,
		"db":    1,
		"event": 2,
	}
)

const (
	testFuncType = `package func_code
	type TypeFunc func(a string, b, c int) (float64, error)`

	testFuncTypeWithDeps = `package func_code
	type TypeFunc func(a string, b db.Connect, c, d int) (*api.Request, error)`

	testEmptyFuncType = `package func_code
	type TypeFunc func()
	`
)

const (
	testStructType = `package struct_code
	type BuiltInStruct struct {
		F string
		V int
	}
	
	type TypeStruct struct {
		A    *api.Request
		B, C int
		d    float32
		E    struct {
			A1     db.Connect
			B1, b2 string
		}
		BuiltInStruct
	}
	`
)

func TestNewStructTypeObj(t *testing.T) {
	t.Run("valid: test struct obj creation with fields and dependencies", func(t *testing.T) {
		structTypeObj := createStructTypeObj(t, testStructType)
		assert.NotNil(t, structTypeObj)

		t.Run("valid: checking struct fields", func(t *testing.T) {
			assert.Equal(t, 5, len(structTypeObj.Fields.List), "field array length")
			assert.Equal(t, 6, structTypeObj.Fields.Len(), "actual number of fields")

			assert.Equal(t, 1, len(structTypeObj.Fields.List[0].Names))
			assert.Equal(t, "A", structTypeObj.Fields.List[0].Names[0].Name)
			assert.Equal(t, true, structTypeObj.Fields.List[0].Names[0].IsExported())
			assert.Equal(t, "Request", structTypeObj.Fields.List[0].Type)

			assert.Equal(t, 2, len(structTypeObj.Fields.List[1].Names))
			assert.Equal(t, "B", structTypeObj.Fields.List[1].Names[0].Name)
			assert.Equal(t, true, structTypeObj.Fields.List[1].Names[0].IsExported())
			assert.Equal(t, "C", structTypeObj.Fields.List[1].Names[1].Name)
			assert.Equal(t, true, structTypeObj.Fields.List[1].Names[1].IsExported())
			assert.Equal(t, "int", structTypeObj.Fields.List[1].Type)

			assert.Equal(t, 1, len(structTypeObj.Fields.List[2].Names))
			assert.Equal(t, "d", structTypeObj.Fields.List[2].Names[0].Name)
			assert.Equal(t, false, structTypeObj.Fields.List[2].Names[0].IsExported())
			assert.Equal(t, "float32", structTypeObj.Fields.List[2].Type)

			assert.Equal(t, 1, len(structTypeObj.Fields.List[3].Names))
			assert.Equal(t, "E", structTypeObj.Fields.List[3].Names[0].Name)
			assert.Equal(t, true, structTypeObj.Fields.List[3].Names[0].IsExported())
			assert.IsType(t, new(StructTypeObj), structTypeObj.Fields.List[3].Type)

			t.Run("valid: checking built-in struct", func(t *testing.T) {
				builtInStructTypeObj, ok := structTypeObj.Fields.List[3].Type.(*StructTypeObj)
				assert.True(t, ok)
				assert.NotNil(t, builtInStructTypeObj)

				t.Run("valid: checking struct fields", func(t *testing.T) {
					assert.Equal(t, 1, len(builtInStructTypeObj.Fields.List[0].Names))
					assert.Equal(t, "A1", builtInStructTypeObj.Fields.List[0].Names[0].Name)
					assert.Equal(t, true, builtInStructTypeObj.Fields.List[0].Names[0].IsExported())
					assert.Equal(t, "Connect", builtInStructTypeObj.Fields.List[0].Type)

					assert.Equal(t, 2, len(builtInStructTypeObj.Fields.List[1].Names))
					assert.Equal(t, "B1", builtInStructTypeObj.Fields.List[1].Names[0].Name)
					assert.Equal(t, true, builtInStructTypeObj.Fields.List[1].Names[0].IsExported())
					assert.Equal(t, "b2", builtInStructTypeObj.Fields.List[1].Names[1].Name)
					assert.Equal(t, false, builtInStructTypeObj.Fields.List[1].Names[1].IsExported())
					assert.Equal(t, "string", builtInStructTypeObj.Fields.List[1].Type)

				})

				t.Run("valid: checking struct dependencies", func(t *testing.T) {
					assert.Equal(t, 1, len(builtInStructTypeObj.DependsParams.List))

					assert.Equal(t, 1, len(builtInStructTypeObj.DependsParams.List[0].Names))
					assert.Equal(t, "db", builtInStructTypeObj.DependsParams.List[0].Names[0].Name)
					assert.Equal(t, "Connect", builtInStructTypeObj.DependsParams.List[0].Type)
				})
			})

			assert.Equal(t, 0, len(structTypeObj.Fields.List[4].Names))
			assert.IsType(t, "BuiltInStruct", structTypeObj.Fields.List[4].Type)
		})
	})

}

func TestNewFuncTypeObj(t *testing.T) {
	t.Run("valid: test empty type func", func(t *testing.T) {
		funcTypeObj := createFuncTypeObj(t, testEmptyFuncType)
		assert.NotNil(t, funcTypeObj)

		t.Run("valid: checking function params", func(t *testing.T) {
			assert.Nil(t, funcTypeObj.Params)
			assert.Nil(t, funcTypeObj.DependsParams)
			assert.Nil(t, funcTypeObj.TypeParams)
			assert.Nil(t, funcTypeObj.ResultParams)
		})
	})

	t.Run("valid: test type func with dependencies ", func(t *testing.T) {
		funcTypeObj := createFuncTypeObj(t, testFuncTypeWithDeps)
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
		funcTypeObj := createFuncTypeObj(t, testFuncType)

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

			if typeSpec.Name.Name == testFuncTypeName {
				funcType, _ = typeSpec.Type.(*ast.FuncType)
				break
			}
		}
	}

	if funcType != nil {
		return funcType
	}

	t.Fatalf("failed: cant find %s in source code", testFuncTypeName)
	return nil
}

func createStructTypeObj(t *testing.T, source string) *StructTypeObj {
	t.Helper()

	fset := token.NewFileSet()
	fobj := NewFileObj(fset, testModuleName, testFileName)
	fobj.Imports = &importTree{Cache: testImportMeta}
	structType := createStructType(t, fset, source)
	structTypeObj, err := NewStructTypeObj(fobj, structType)
	assert.NoError(t, err)

	return structTypeObj
}

func createStructType(t *testing.T, fset *token.FileSet, source string) *ast.StructType {
	t.Helper()

	file, err := parser.ParseFile(fset, "", strings.NewReader(source), parser.AllErrors)
	require.NoError(t, err)
	require.NotNil(t, file)

	var structType *ast.StructType

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

			if typeSpec.Name.Name == testStructTypeName {
				structType, _ = typeSpec.Type.(*ast.StructType)
				break
			}
		}
	}

	if structType != nil {
		return structType
	}

	t.Fatalf("failed: cant find %s in source code", testStructTypeName)
	return nil
}
