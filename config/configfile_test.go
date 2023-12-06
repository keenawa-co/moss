package config

import (
	"errors"
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

const (
	interpolatedStr = "INTERPOLATED!"
)

func testInterpolateMock(lookup envLookupFunc, strWithEnvs string) (string, error) {
	return interpolatedStr, nil
}

func testInterpolateErroneousMock(lookup envLookupFunc, strWithEnvs string) (string, error) {
	return "", errors.New("an error has happened")
}

func TestReplaceEnvValues(t *testing.T) {
	t.Run("valid: integer returned 'as-is'", func(t *testing.T) {
		got, err := replaceEnvValues(testLookup, testInterpolateMock, 123)

		assert.Equal(t, 123, got)
		assert.Nil(t, err)
	})

	t.Run("valid: string interpolated 'as-is'", func(t *testing.T) {
		got, err := replaceEnvValues(testLookup, testInterpolateMock, "test")

		assert.Equal(t, interpolatedStr, got)
		assert.Nil(t, err)
	})

	t.Run("valid: empty string returned 'as-is'", func(t *testing.T) {
		got, err := replaceEnvValues(testLookup, testInterpolateMock, "")

		assert.Equal(t, "", got)
		assert.Nil(t, err)
	})

	t.Run("valid: int slice returned 'as-is'", func(t *testing.T) {
		s := make([]int, 5)
		s[0] = 5
		s[1] = 10

		got, err := replaceEnvValues(testLookup, testInterpolateMock, s)

		assert.Equal(t, s, got)
		assert.Nil(t, err)
	})

	t.Run("valid: string map is interpolated", func(t *testing.T) {
		const size int = 5

		s := make([]string, size)
		expected := make([]string, size)

		// fill both slices with values
		const defaultStr string = "default value"
		for i := 0; i < size; i++ {
			s[i] = defaultStr
			expected[i] = interpolatedStr
		}

		_, err := replaceEnvValues(testLookup, testInterpolateMock, s)

		assert.Equal(t, expected, s)
		assert.Nil(t, err)
	})
}
