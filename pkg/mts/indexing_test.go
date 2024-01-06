package mts

import (
	"reflect"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestNewIndexTable(t *testing.T) {
	t.Run("Create New IndexTable", func(t *testing.T) {
		idx := newIndexTable()
		assert.NotNil(t, idx, "New indexTable should not be nil")
		assert.Empty(t, idx.table, "New indexTable should be empty")
	})
}

func TestIndexTable(t *testing.T) {
	t.Run("Store operation", func(t *testing.T) {
		idx := newIndexTable()
		testOrigin := &origin{name: "test"}
		idx.store("testKey", testOrigin)

		assert.NotEmpty(t, idx.table, "Table should not be empty after store operation")
		assert.Equal(t, testOrigin, idx.table["testKey"], "Stored value should be equal to the inserted value")
	})

	t.Run("Load existing key", func(t *testing.T) {
		idx := newIndexTable()
		testOrigin := &origin{name: "test"}
		idx.store("testKey", testOrigin)

		loadedValue, exists := idx.load("testKey")
		assert.True(t, exists, "Key should exist")
		assert.Equal(t, testOrigin, loadedValue, "Loaded value should be equal to the inserted value")
	})

	t.Run("Load non-existing key", func(t *testing.T) {
		idx := newIndexTable()

		_, exists := idx.load("nonExistingKey")
		assert.False(t, exists, "Key should not exist")
	})

	t.Run("Debug empty table", func(t *testing.T) {
		idx := newIndexTable()
		assert.Equal(t, "{}", idx.debug(), "Debug of empty table should return {}")
	})

	t.Run("Debug non-empty table", func(t *testing.T) {
		idx := newIndexTable()
		testOrigin := &origin{name: "test"}
		idx.store("testKey", testOrigin)

		assert.NotEqual(t, "{}", idx.debug(), "Debug of non-empty table should not return {}")
	})
}

func TestSourceMapIndexing(t *testing.T) {
	t.Run("Empty map should return empty indexTable", func(t *testing.T) {
		emptyMap := make(map[string]interface{})
		val := reflect.ValueOf(emptyMap)

		idxt := sourceMapIndexing(val, "")
		assert.NotNil(t, idxt, "Returned indexTable should not be nil")
		assert.Empty(t, idxt.table, "IndexTable of empty map should be empty")
	})

	t.Run("Single element map should return indexTable with one entry", func(t *testing.T) {
		singleMap := map[string]interface{}{"key": "value"}
		val := reflect.ValueOf(singleMap)

		idxt := sourceMapIndexing(val, "")
		assert.NotNil(t, idxt, "Returned indexTable should not be nil")
		assert.Len(t, idxt.table, 1, "IndexTable of single element map should have one entry")
		assert.Contains(t, idxt.table, "key", "IndexTable should contain the key")
	})

	t.Run("Nested map should return indexTable with entries for each element", func(t *testing.T) {
		nestedMap := map[string]interface{}{"parent": map[string]interface{}{"child": "value"}}
		val := reflect.ValueOf(nestedMap)

		idxt := sourceMapIndexing(val, "")
		assert.NotNil(t, idxt, "Returned indexTable should not be nil")
		assert.Len(t, idxt.table, 2, "IndexTable of nested map should have entries for each element")
		assert.Contains(t, idxt.table, "parent", "IndexTable should contain the parent key")
		assert.Contains(t, idxt.table, "parent.child", "IndexTable should contain the nested child key")
	})
}

func TestSourceSliceIndexing(t *testing.T) {
	t.Run("Empty slice should return empty indexTable", func(t *testing.T) {
		emptySlice := make([]map[string]interface{}, 0)
		val := reflect.ValueOf(emptySlice)

		idxt := sourceSliceIndexing(val, "")
		assert.NotNil(t, idxt, "Returned indexTable should not be nil")
		assert.Empty(t, idxt.table, "IndexTable of empty slice should be empty")
	})

	t.Run("Slice with single map element should return indexTable with entries", func(t *testing.T) {
		singleSlice := []map[string]interface{}{{"key": "value"}}
		val := reflect.ValueOf(singleSlice)

		idxt := sourceSliceIndexing(val, "")
		assert.NotNil(t, idxt, "Returned indexTable should not be nil")

		assert.NotEmpty(t, idxt.table, "IndexTable of slice with single map element should not be empty")
	})

	t.Run("Slice with multiple map elements should return indexTable with all entries", func(t *testing.T) {
		multipleSlice := []map[string]interface{}{
			{"key1": "value1"},
			{"key2": "value2"},
		}
		val := reflect.ValueOf(multipleSlice)

		idxt := sourceSliceIndexing(val, "")
		assert.NotNil(t, idxt, "Returned indexTable should not be nil")
		assert.NotEmpty(t, idxt.table, "IndexTable of slice with multiple map elements should not be empty")
		assert.GreaterOrEqual(t, len(idxt.table), 2, "IndexTable should contain entries for all map elements")
	})
}

func TestDestStructIndexing(t *testing.T) {
	t.Run("Indexing empty struct", func(t *testing.T) {
		emptyStruct := struct{}{}
		val := reflect.ValueOf(emptyStruct)
		idxt := destStructIndexing(newIndexTable(), val, "", "json")

		assert.NotNil(t, idxt, "Returned indexTable should not be nil")
		assert.Empty(t, idxt.table, "IndexTable of empty struct should be empty")
	})

	t.Run("Indexing simple struct", func(t *testing.T) {
		simpleStruct := struct {
			FieldOne string `json:"field_one"`
		}{
			FieldOne: "ValueOne",
		}
		val := reflect.ValueOf(simpleStruct)
		idxt := destStructIndexing(newIndexTable(), val, "", "json")

		assert.NotNil(t, idxt, "Returned indexTable should not be nil")
		assert.NotEmpty(t, idxt.table, "IndexTable of simple struct should not be empty")
		assert.Contains(t, idxt.table, "field_one", "IndexTable should contain key for 'field_one'")
	})

	t.Run("Indexing nested struct", func(t *testing.T) {
		type NestedStruct struct {
			NestedField string `json:"nested_field"`
		}
		type ParentStruct struct {
			Nested NestedStruct `json:"nested"`
		}
		nestedStruct := ParentStruct{
			Nested: NestedStruct{
				NestedField: "NestedValue",
			},
		}
		val := reflect.ValueOf(nestedStruct)
		idxt := destStructIndexing(newIndexTable(), val, "", "json")

		assert.NotNil(t, idxt, "Returned indexTable should not be nil")
		assert.NotEmpty(t, idxt.table, "IndexTable of nested struct should not be empty")
		assert.Contains(t, idxt.table, "nested.nested_field", "IndexTable should contain key for 'nested.nested_field'")
	})

	t.Run("Indexing struct with pointers", func(t *testing.T) {
		type StructWithPointers struct {
			PtrField *string `json:"ptr_field"`
		}
		val := "Value"
		testStruct := StructWithPointers{
			PtrField: &val,
		}
		valReflect := reflect.ValueOf(testStruct)
		idxt := destStructIndexing(newIndexTable(), valReflect, "", "json")

		assert.NotNil(t, idxt, "Returned indexTable should not be nil")
		assert.NotEmpty(t, idxt.table, "IndexTable of struct with pointers should not be empty")
		assert.Contains(t, idxt.table, "ptr_field", "IndexTable should contain key for 'ptr_field'")
	})

	t.Run("Indexing struct with slices", func(t *testing.T) {
		type StructWithSlices struct {
			SliceField []string `json:"slice_field"`
		}
		testStruct := StructWithSlices{
			SliceField: []string{"Item1", "Item2"},
		}
		valReflect := reflect.ValueOf(testStruct)
		idxt := destStructIndexing(newIndexTable(), valReflect, "", "json")

		assert.NotNil(t, idxt, "Returned indexTable should not be nil")
		assert.NotEmpty(t, idxt.table, "IndexTable of struct with slices should not be empty")
		assert.Contains(t, idxt.table, "slice_field", "IndexTable should contain key for 'slice_field'")
	})

	t.Run("Indexing struct with empty fields", func(t *testing.T) {
		type StructWithEmptyFields struct {
			EmptyString string            `json:"empty_string"`
			EmptySlice  []int             `json:"empty_slice"`
			EmptyMap    map[string]string `json:"empty_map"`
		}
		testStruct := StructWithEmptyFields{}
		valReflect := reflect.ValueOf(testStruct)
		idxt := destStructIndexing(newIndexTable(), valReflect, "", "json")

		assert.NotNil(t, idxt, "Returned indexTable should not be nil")
		assert.NotEmpty(t, idxt.table, "IndexTable should not be empty even with empty fields")
		assert.Contains(t, idxt.table, "empty_string", "IndexTable should contain key for 'empty_string'")
		assert.Contains(t, idxt.table, "empty_slice", "IndexTable should contain key for 'empty_slice'")
		assert.Contains(t, idxt.table, "empty_map", "IndexTable should contain key for 'empty_map'")
	})

	t.Run("Struct with map of structs as values", func(t *testing.T) {
		type MapValueStruct struct {
			ValueField string `json:"value_field"`
		}
		type StructWithMap struct {
			MapField map[string]MapValueStruct `json:"map_field"`
		}
		testStruct := StructWithMap{
			MapField: map[string]MapValueStruct{"key": {ValueField: "Value"}},
		}
		valReflect := reflect.ValueOf(testStruct)
		idxt := destStructIndexing(newIndexTable(), valReflect, "", "json")

		assert.NotNil(t, idxt, "Returned indexTable should not be nil")
		assert.NotEmpty(t, idxt.table, "IndexTable should not be empty for struct with map of structs")
		assert.Contains(t, idxt.table, "map_field.key.value_field", "IndexTable should contain keys for fields of structs in the map")
	})

	t.Run("Struct with nested pointers to structs", func(t *testing.T) {
		type NestedStruct struct {
			NestedField string `json:"nested_field"`
		}

		type NestedPtrStruct struct {
			NestedPtr *NestedStruct `json:"nested_ptr"`
		}

		nestedValue := "NestedValue"
		testStruct := NestedPtrStruct{
			NestedPtr: &NestedStruct{NestedField: nestedValue},
		}
		valReflect := reflect.ValueOf(testStruct)
		idxt := destStructIndexing(newIndexTable(), valReflect, "", "json")

		assert.NotNil(t, idxt, "Returned indexTable should not be nil")
		assert.NotEmpty(t, idxt.table, "IndexTable should not be empty for struct with nested pointers to structs")
		assert.Contains(t, idxt.table, "nested_ptr.nested_field", "IndexTable should contain keys for fields of nested structs")
	})
}

func TestDestSliceIndexing(t *testing.T) {
	t.Run("Indexing empty slice", func(t *testing.T) {
		emptySlice := []struct{}{}
		val := reflect.ValueOf(&emptySlice).Elem()
		sit := newIndexTable()
		idxt := destSliceIndexing(sit, val, "path.to.slice", "json")

		assert.Nil(t, idxt, "Returned indexTable should not be nil")
	})
}

func TestGetFieldTag(t *testing.T) {
	type SampleStruct struct {
		FieldWithoutTag   string
		FieldWithTag      string `json:"field_with_tag"`
		FieldWithEmptyTag string `json:""`
	}

	typ := reflect.TypeOf(SampleStruct{})

	t.Run("Field without tag", func(t *testing.T) {
		field, _ := typ.FieldByName("FieldWithoutTag")
		tag := getFieldTag(field, "json")
		assert.Equal(t, "FieldWithoutTag", tag, "Should return the field name when no tag is present")
	})

	t.Run("Field with tag", func(t *testing.T) {
		field, _ := typ.FieldByName("FieldWithTag")
		tag := getFieldTag(field, "json")
		assert.Equal(t, "field_with_tag", tag, "Should return the tag value when tag is present")
	})

	t.Run("Field with empty tag", func(t *testing.T) {
		field, _ := typ.FieldByName("FieldWithEmptyTag")
		tag := getFieldTag(field, "json")
		assert.Equal(t, "FieldWithEmptyTag", tag, "Should return the field name when tag is empty")
	})
}

func TestDeRef(t *testing.T) {
	t.Run("DeRef non-pointer value", func(t *testing.T) {
		original := 42
		val := reflect.ValueOf(original)
		result := deRef(val)

		assert.Equal(t, val.Interface(), result.Interface(), "DeRef should return the original value for non-pointers")
	})

	t.Run("DeRef nil pointer", func(t *testing.T) {
		var original *int
		val := reflect.ValueOf(original)
		result := deRef(val)

		assert.Equal(t, val.Interface(), result.Interface(), "DeRef should return the original nil pointer without dereferencing")
	})

	t.Run("DeRef valid pointer", func(t *testing.T) {
		original := 42
		val := reflect.ValueOf(&original)
		result := deRef(val)

		assert.Equal(t, original, result.Interface(), "DeRef should return the dereferenced value for non-nil pointers")
	})
}

func TestBuildPath(t *testing.T) {
	t.Run("Non-empty base path", func(t *testing.T) {
		result := buildPath("base", "key")
		assert.Equal(t, "base.key", result)
	})

	t.Run("Empty base path", func(t *testing.T) {
		result := buildPath("", "key")
		assert.Equal(t, "key", result)
	})

	t.Run("Both empty base path and key", func(t *testing.T) {
		result := buildPath("", "")
		assert.Equal(t, "", result)
	})

	t.Run("Non-empty base path and empty key", func(t *testing.T) {
		result := buildPath("base", "")
		assert.Equal(t, "base.", result)
	})
}

func TestBuildSliceElemPath(t *testing.T) {
	t.Run("Non-empty path", func(t *testing.T) {
		result := buildSliceElemPath("path", 1)
		assert.Equal(t, "path.[1]", result)
	})

	t.Run("Empty path", func(t *testing.T) {
		result := buildSliceElemPath("", 1)
		assert.Equal(t, ".[1]", result)
	})

	t.Run("Zero index", func(t *testing.T) {
		result := buildSliceElemPath("path", 0)
		assert.Equal(t, "path.[0]", result)
	})

	t.Run("Negative index", func(t *testing.T) {
		result := buildSliceElemPath("path", -1)
		assert.Equal(t, "path.[-1]", result)
	})
}
