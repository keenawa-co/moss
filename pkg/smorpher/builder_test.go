package smorpher

import (
	"fmt"
	"reflect"
	"testing"

	"github.com/stretchr/testify/assert"
)

type testEmbedArrayItem struct {
	FieldOne      string               `json:"field_one"`
	FieldTwo      string               `json:"field_two"`
	NormalSet     []string             `json:"normal_set"`
	EmbedArrayTwo []testEmbedArrayItem `json:"embed_array_two"`
}

type testEmbedStruct struct {
	FieldOne         string                `json:"field_one"`
	FieldTwo         string                `json:"field_two"`
	NormalSet        []string              `json:"normal_set"`
	AutocompletedSet []string              `json:"autocompleted_set"`
	EmbedArrayOne    []*testEmbedArrayItem `json:"embed_array_one"`
}

type testStruct struct {
	FieldOne string           `json:"field_one"`
	FieldTwo string           `json:"field_two"`
	Embed    *testEmbedStruct `json:"embed"`
}

var (
	testDataFieldOne          = "field_one_data"
	testDataFieldTwo          = "field_two_data"
	testDataSet               = []string{"foo", "bar", "zip", "zap"}
	testDataEmbedArrayItemOne = map[string]interface{}{
		"field_one":  testDataFieldOne,
		"field_two":  &testDataFieldTwo,
		"normal_set": testDataSet,
		"embed_array_two": []map[string]interface{}{
			{
				"field_one": testDataFieldOne,
			},
		},
	}
	testDataEmbedArrayItemTwo = map[string]interface{}{
		"field_one": testDataFieldOne,
		"field_two": &testDataFieldTwo,
	}
	testDataEmbedArrayItemThree = map[string]interface{}{
		"field_one": testDataFieldOne,
	}
	testDataEmbedArray = []map[string]interface{}{
		testDataEmbedArrayItemOne,
		testDataEmbedArrayItemTwo,
		testDataEmbedArrayItemThree,
	}

	testEmptyData = map[string]interface{}{}
	testData      = map[string]interface{}{
		"field_one": testDataFieldOne,
		"field_two": &testDataFieldTwo,
		"embed": map[string]interface{}{
			"field_one":         testDataFieldOne,
			"field_two":         &testDataFieldTwo,
			"normal_set":        testDataSet,
			"autocompleted_set": testDataFieldOne,
			"embed_array_one":   testDataEmbedArray,
		},
	}
)

func TestNewBuilder(t *testing.T) {
	t.Run("Create struct builder from empty source data", func(t *testing.T) {
		destStruct := new(testStruct)
		b, err := NewBuilder(destStruct, testEmptyData)
		assert.NoError(t, err)
		assert.NotNil(t, b)
	})

	t.Run("Create struct builder from source data", func(t *testing.T) {
		destStruct := new(testStruct)
		b, err := NewBuilder(destStruct, testData)
		assert.NoError(t, err)
		assert.NotNil(t, b)
	})
}

func TestBuilder(t *testing.T) {
	t.Run("Build destination pointer struct without autocomplete", func(t *testing.T) {
		destStruct := new(testStruct)
		b, _ := NewBuilder(destStruct, testData)

		Walk(b, &Field{
			Path:  nil,
			Value: testData,
			Kind:  reflect.ValueOf(testData).Kind(),
		})

		assert.NotNil(t, destStruct)
		assert.Equal(t, testDataFieldOne, destStruct.FieldOne)
		assert.Equal(t, testDataFieldTwo, destStruct.FieldTwo)
		assert.NotNil(t, destStruct.Embed)

		assert.Equal(t, testDataFieldOne, destStruct.Embed.FieldOne)
		assert.Equal(t, testDataFieldTwo, destStruct.Embed.FieldTwo)
		assert.NotNil(t, destStruct.Embed.NormalSet)
		assert.Equal(t, testDataSet, destStruct.Embed.NormalSet)
		assert.Equal(t, []string(nil), destStruct.Embed.AutocompletedSet)
		assert.NotNil(t, destStruct.Embed.EmbedArrayOne)
		assert.Len(t, destStruct.Embed.EmbedArrayOne, len(testDataEmbedArray))

		assert.Equal(t, testDataFieldOne, destStruct.Embed.EmbedArrayOne[0].FieldOne)
		assert.Equal(t, testDataFieldTwo, destStruct.Embed.EmbedArrayOne[0].FieldTwo)
		assert.NotNil(t, destStruct.Embed.EmbedArrayOne[0].NormalSet)
		assert.Equal(t, testDataSet, destStruct.Embed.EmbedArrayOne[0].NormalSet)
		assert.NotNil(t, destStruct.Embed.EmbedArrayOne[0].EmbedArrayTwo)
		assert.Len(t, destStruct.Embed.EmbedArrayOne[0].EmbedArrayTwo, len(testDataEmbedArrayItemOne["embed_array_two"].([]map[string]interface{})))

		assert.Equal(t, testDataFieldOne, destStruct.Embed.EmbedArrayOne[1].FieldOne)
		assert.Equal(t, testDataFieldTwo, destStruct.Embed.EmbedArrayOne[1].FieldTwo)
		assert.Nil(t, destStruct.Embed.EmbedArrayOne[1].NormalSet)

		assert.Equal(t, testDataFieldOne, destStruct.Embed.EmbedArrayOne[2].FieldOne)
		assert.Empty(t, destStruct.Embed.EmbedArrayOne[2].FieldTwo)
		assert.Nil(t, destStruct.Embed.EmbedArrayOne[2].NormalSet)
	})

	t.Run("Build destination struct without autocomplete", func(t *testing.T) {
		destStruct := testStruct{}
		b, _ := NewBuilder(destStruct, testData)

		Walk(b, &Field{
			Path:  nil,
			Value: testData,
			Kind:  reflect.ValueOf(testData).Kind(),
		})

		assert.NotNil(t, destStruct)
		assert.Equal(t, testDataFieldOne, destStruct.FieldOne)
		assert.Equal(t, testDataFieldTwo, destStruct.FieldTwo)
		assert.NotNil(t, destStruct.Embed)

		assert.Equal(t, testDataFieldOne, destStruct.Embed.FieldOne)
		assert.Equal(t, testDataFieldTwo, destStruct.Embed.FieldTwo)
		assert.NotNil(t, destStruct.Embed.NormalSet)
		assert.Equal(t, testDataSet, destStruct.Embed.NormalSet)
		assert.Equal(t, []string(nil), destStruct.Embed.AutocompletedSet)
		assert.NotNil(t, destStruct.Embed.EmbedArrayOne)
		assert.Len(t, destStruct.Embed.EmbedArrayOne, len(testDataEmbedArray))

		assert.Equal(t, testDataFieldOne, destStruct.Embed.EmbedArrayOne[0].FieldOne)
		assert.Equal(t, testDataFieldTwo, destStruct.Embed.EmbedArrayOne[0].FieldTwo)
		assert.NotNil(t, destStruct.Embed.EmbedArrayOne[0].NormalSet)
		assert.Equal(t, testDataSet, destStruct.Embed.EmbedArrayOne[0].NormalSet)
		assert.NotNil(t, destStruct.Embed.EmbedArrayOne[0].EmbedArrayTwo)
		assert.Len(t, destStruct.Embed.EmbedArrayOne[0].EmbedArrayTwo, len(testDataEmbedArrayItemOne["embed_array_two"].([]map[string]interface{})))

		assert.Equal(t, testDataFieldOne, destStruct.Embed.EmbedArrayOne[1].FieldOne)
		assert.Equal(t, testDataFieldTwo, destStruct.Embed.EmbedArrayOne[1].FieldTwo)
		assert.Nil(t, destStruct.Embed.EmbedArrayOne[1].NormalSet)

		assert.Equal(t, testDataFieldOne, destStruct.Embed.EmbedArrayOne[2].FieldOne)
		assert.Empty(t, destStruct.Embed.EmbedArrayOne[2].FieldTwo)
		assert.Nil(t, destStruct.Embed.EmbedArrayOne[2].NormalSet)
	})

	t.Run("Build destination pointer struct with autocomplete", func(t *testing.T) {
		destStruct := new(testStruct)
		b, _ := NewBuilder(destStruct, testData, WithMode(Autocomplete))

		Walk(b, &Field{
			Path:  nil,
			Value: testData,
			Kind:  reflect.ValueOf(testData).Kind(),
		})

		assert.NotNil(t, destStruct)
		assert.Equal(t, testDataFieldOne, destStruct.FieldOne)
		assert.Equal(t, testDataFieldTwo, destStruct.FieldTwo)
		assert.NotNil(t, destStruct.Embed)

		assert.Equal(t, testDataFieldOne, destStruct.Embed.FieldOne)
		assert.Equal(t, testDataFieldTwo, destStruct.Embed.FieldTwo)
		assert.NotNil(t, destStruct.Embed.NormalSet)
		assert.Equal(t, testDataSet, destStruct.Embed.NormalSet)
		assert.Equal(t, []string{testDataFieldOne}, destStruct.Embed.AutocompletedSet)
		assert.NotNil(t, destStruct.Embed.EmbedArrayOne)
		assert.Len(t, destStruct.Embed.EmbedArrayOne, len(testDataEmbedArray))

		assert.Equal(t, testDataFieldOne, destStruct.Embed.EmbedArrayOne[0].FieldOne)
		assert.Equal(t, testDataFieldTwo, destStruct.Embed.EmbedArrayOne[0].FieldTwo)
		assert.NotNil(t, destStruct.Embed.EmbedArrayOne[0].NormalSet)
		assert.Equal(t, testDataSet, destStruct.Embed.EmbedArrayOne[0].NormalSet)
		assert.NotNil(t, destStruct.Embed.EmbedArrayOne[0].EmbedArrayTwo)
		assert.Len(t, destStruct.Embed.EmbedArrayOne[0].EmbedArrayTwo, len(testDataEmbedArrayItemOne["embed_array_two"].([]map[string]interface{})))

		assert.Equal(t, testDataFieldOne, destStruct.Embed.EmbedArrayOne[1].FieldOne)
		assert.Equal(t, testDataFieldTwo, destStruct.Embed.EmbedArrayOne[1].FieldTwo)
		assert.Nil(t, destStruct.Embed.EmbedArrayOne[1].NormalSet)

		assert.Equal(t, testDataFieldOne, destStruct.Embed.EmbedArrayOne[2].FieldOne)
		assert.Empty(t, destStruct.Embed.EmbedArrayOne[2].FieldTwo)
		assert.Nil(t, destStruct.Embed.EmbedArrayOne[2].NormalSet)
	})
}

func TestBuilder2(t *testing.T) {
	user := "g10z3r"
	version := "1.1"
	m := map[string]interface{}{
		"version": &version,
		"user":    user,

		"embed": map[string]interface{}{
			"term":       "xterm-256color",
			"path":       "src/dir",
			"normal_set": []string{"foo", "bar", "zip", "zap"},
			"embed_array_one": []map[string]interface{}{
				{
					"field_one":  testDataFieldOne,
					"field_two":  &testDataFieldTwo,
					"normal_set": testDataSet,
					"embed_array_two": []map[string]interface{}{
						{
							"path": "not-root",
						},
					},
				},
				{
					"path": "not-root",
				},
				{
					"path": "foo",
				},
			},
		},
	}

	ts := &testStruct{}
	v, err := NewBuilder(ts, m, WithMode(Autocomplete))
	if err != nil {
		fmt.Println(err)
	}

	Walk(v, &Field{
		Path:  nil,
		Value: m,
		Kind:  reflect.ValueOf(m).Kind(),
	})

	fmt.Println("version", ts.FieldOne)
	fmt.Println("user", ts.FieldTwo)
	fmt.Println("set, pointer to string", ts.Embed.NormalSet)
	fmt.Println(ts.Embed.EmbedArrayOne[0].EmbedArrayTwo[0].FieldOne)
	fmt.Println("targets, string to pointer", ts.Embed.EmbedArrayOne[0].NormalSet)
	fmt.Println(ts.Embed.EmbedArrayOne[1])
	t.Fail()
}
