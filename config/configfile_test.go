package config

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func testLookup(key string) (string, bool) {
	mockEnv := map[string]string{
		"GORAY":         "some_value",
		"THREADS_COUNT": "5",
	}

	value, exists := mockEnv[key]
	return value, exists
}

func TestInterpolate(t *testing.T) {
	// TODO: unit for interpolate
	t.Run("valid: one env variable found", func(t *testing.T) {
		got, err := interpolate(testLookup, "${GORAY}")

		assert.Equal(t, "some_value", got)
		assert.Nil(t, err)
	})

	t.Run("valid: one env variable found within the same string", func(t *testing.T) {
		got, err := interpolate(testLookup, "abc ${GORAY} abc")

		assert.Equal(t, "abc some_value abc", got)
		assert.Nil(t, err)
	})

	t.Run("valid: two env variables found within the same string", func(t *testing.T) {
		got, err := interpolate(testLookup, "${GORAY}${THREADS_COUNT}")

		assert.Equal(t, "some_value5", got)
		assert.Nil(t, err)
	})

	t.Run("valid: two env variables found within the same string", func(t *testing.T) {
		got, err := interpolate(testLookup, "${GORAY}${THREADS_COUNT}")

		assert.Equal(t, "some_value5", got)
		assert.Nil(t, err)
	})

	t.Run("valid: no env variables within the string", func(t *testing.T) {
		got, err := interpolate(testLookup, "${NOT AN ENV")

		assert.Equal(t, "${NOT AN ENV", got)
		assert.Nil(t, err)
	})

	t.Run("invalid: env variable not found", func(t *testing.T) {
		got, err := interpolate(testLookup, "${SOME_NAME}")

		assert.Equal(t, "", got)
		assert.Equal(t, "environment variables not found: SOME_NAME", err.Error())
	})

	t.Run("invalid: one env variable found, one not found", func(t *testing.T) {
		got, err := interpolate(testLookup, "${GORAY}${SOME_NAME}")

		assert.Equal(t, "", got)
		assert.Equal(t, "environment variables not found: SOME_NAME", err.Error())
	})

	t.Run("invalid: multiple env variables not found", func(t *testing.T) {
		got, err := interpolate(testLookup, "${SOME_NAME1} ${SOME_NAME2} ${SOME_NAME3}")

		assert.Equal(t, "", got)
		assert.Equal(t, "environment variables not found: SOME_NAME1, SOME_NAME2, SOME_NAME3", err.Error())
	})
}

func testGetConfigFileContentMap() cfgMap {
	workspace := make(cfgMap)
	workspace["root"] = "${GORAY}"

	ignoreList := make([]interface{}, 3)
	ignoreList = append(ignoreList, "first value")
	ignoreList = append(ignoreList, "string with env: ${THREADS_COUNT}")
	ignoreList = append(ignoreList, "third value")

	workspace["ignore_list"] = ignoreList

	m := make(cfgMap)
	m["workspace"] = workspace

	return m

}
