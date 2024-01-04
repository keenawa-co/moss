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
				"path": "not-root",
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
		assert.Equal(t, destStruct.FieldOne, testDataFieldOne)
		assert.Equal(t, destStruct.FieldTwo, testDataFieldTwo)
		assert.NotNil(t, destStruct.Embed)

		assert.Equal(t, destStruct.Embed.FieldOne, testDataFieldOne)
		assert.Equal(t, destStruct.Embed.FieldTwo, testDataFieldTwo)
		assert.NotNil(t, destStruct.Embed.NormalSet)
		assert.Equal(t, destStruct.Embed.NormalSet, testDataSet)
		assert.Equal(t, destStruct.Embed.AutocompletedSet, []string(nil))
		assert.NotNil(t, destStruct.Embed.EmbedArrayOne)
		assert.Len(t, destStruct.Embed.EmbedArrayOne, len(testDataEmbedArray))

		assert.Equal(t, destStruct.Embed.EmbedArrayOne[0].FieldOne, testDataFieldOne)
		assert.Equal(t, destStruct.Embed.EmbedArrayOne[0].FieldTwo, testDataFieldTwo)
		assert.NotNil(t, destStruct.Embed.EmbedArrayOne[0].NormalSet)
		assert.Equal(t, destStruct.Embed.EmbedArrayOne[0].NormalSet, testDataSet)
		assert.NotNil(t, destStruct.Embed.EmbedArrayOne[0].EmbedArrayTwo)
		assert.Len(t, destStruct.Embed.EmbedArrayOne[0].EmbedArrayTwo, len(testDataEmbedArrayItemOne["embed_array_two"].([]map[string]interface{})))

		fmt.Println(destStruct.Embed.EmbedArrayOne[1])
		assert.Equal(t, destStruct.Embed.EmbedArrayOne[1].FieldOne, testDataFieldOne)
		// assert.Equal(t, destStruct.Embed.EmbedArrayOne[1].FieldTwo, testDataFieldTwo)
	})
}

func TestBuilder2(t *testing.T) {
	user := "g10z3r"
	version := "1.1"
	m := map[string]interface{}{
		"version": &version,
		"user":    user,

		"data": map[string]interface{}{
			"term":       "xterm-256color",
			"path":       "src/dir",
			"normal_set": []string{"foo", "bar", "zip", "zap"},
			"workspace": []map[string]interface{}{
				{
					"path":    "root",
					"targets": []string{"t1", "t2", "t3"},
					"emm": []map[string]interface{}{
						{
							"k1": "v1",
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

	// fmt.Println("version", ts.FieldOne)
	// fmt.Println("user", ts.FieldTwo)
	// fmt.Println("set, pointer to string", ts.Embed.NormalSet)
	// fmt.Println(ts.Embed.EmbedArrayOne[0].Emm[0].K1)
	// fmt.Println("targets, string to pointer", ts.Embed.EmbedArrayOne[0].Targets)

	t.Fail()
}
