package config

import (
	"errors"
	"testing"

	"github.com/BurntSushi/toml"
	"github.com/stretchr/testify/assert"
)

func testLookupEnv(key string) (string, bool) {
	mockEnv := map[string]string{
		"GORAY":         "some_value",
		"THREADS_COUNT": "5",
	}

	value, exists := mockEnv[key]
	return value, exists
}

func TestInterpolate(t *testing.T) {
	t.Run("valid: one env variable found", func(t *testing.T) {
		got, err := interpolate(testLookupEnv, "${GORAY}")

		assert.Equal(t, "some_value", got)
		assert.Nil(t, err)
	})

	t.Run("valid: one env variable found within the same string", func(t *testing.T) {
		got, err := interpolate(testLookupEnv, "abc ${GORAY} abc")

		assert.Equal(t, "abc some_value abc", got)
		assert.Nil(t, err)
	})

	t.Run("valid: two env variables found within the same string", func(t *testing.T) {
		got, err := interpolate(testLookupEnv, "${GORAY}${THREADS_COUNT}")

		assert.Equal(t, "some_value5", got)
		assert.Nil(t, err)
	})

	t.Run("valid: two env variables found within the same string", func(t *testing.T) {
		got, err := interpolate(testLookupEnv, "${GORAY}${THREADS_COUNT}")

		assert.Equal(t, "some_value5", got)
		assert.Nil(t, err)
	})

	t.Run("valid: no env variables within the string", func(t *testing.T) {
		got, err := interpolate(testLookupEnv, "${NOT AN ENV")

		assert.Equal(t, "${NOT AN ENV", got)
		assert.Nil(t, err)
	})

	t.Run("invalid: env variable not found", func(t *testing.T) {
		got, err := interpolate(testLookupEnv, "${SOME_NAME}")

		assert.Equal(t, "", got)
		assert.Equal(t, "environment variables not found: SOME_NAME", err.Error())
	})

	t.Run("invalid: one env variable found, one not found", func(t *testing.T) {
		got, err := interpolate(testLookupEnv, "${GORAY}${SOME_NAME}")

		assert.Equal(t, "", got)
		assert.Equal(t, "environment variables not found: SOME_NAME", err.Error())
	})

	t.Run("invalid: multiple env variables not found", func(t *testing.T) {
		got, err := interpolate(testLookupEnv, "${SOME_NAME1} ${SOME_NAME2} ${SOME_NAME3}")

		assert.Equal(t, "", got)
		assert.Equal(t, "environment variables not found: SOME_NAME1, SOME_NAME2, SOME_NAME3", err.Error())
	})
}

func withTestReadFile(rc *ReadConf) {
	rc.readFile = testReadFile
}

func testReadFile(name string) ([]byte, error) {
	return []byte("read fide"), nil
}

func withTestReadFileErr(rc *ReadConf) {
	rc.readFile = testReadFileErr
}

func testReadFileErr(name string) ([]byte, error) {
	return nil, errors.New("read file error")
}

func withTestReadToml(rc *ReadConf) {
	rc.readToml = testReadToml
}

func testReadToml(data string, v interface{}) (toml.MetaData, error) {
	return toml.MetaData{}, nil
}

func withTestReadTomlErr(rc *ReadConf) {
	rc.readToml = testReadTomlErr
}

func testReadTomlErr(data string, v interface{}) (toml.MetaData, error) {
	return toml.MetaData{}, errors.New("read toml error")

}

func withTestInterpolate(rc *ReadConf) {
	rc.interpolate = testInterpolate
}

func testInterpolate(lookup LookupFn, strWithEnvs string) (string, error) {
	return "interpolated string", nil
}

func withTestInterpolateErr(rc *ReadConf) {
	rc.interpolate = testInterpolateErr
}

func testInterpolateErr(lookup LookupFn, strWithEnvs string) (string, error) {
	return "", errors.New("interpolate error")
}

func withTestLookup(rc *ReadConf) {
	rc.lookup = testLookup
}

func testLookup(key string) (string, bool) {
	return "", false
}

func TestNewConfigFromFile(t *testing.T) {

	t.Run("valid: default config is returned", func(t *testing.T) {
		got, err := NewConfigFromFile("", withTestReadFile, withTestInterpolate, withTestReadToml, withTestLookup)

		expected := NewConfig()
		assert.Equal(t, expected, got)
		assert.Nil(t, err)
	})

	t.Run("invalid: file read error", func(t *testing.T) {
		got, err := NewConfigFromFile("", withTestReadFileErr)

		assert.Nil(t, nil, got)
		assert.Equal(t, "read file error", err.Error())
	})

	t.Run("invalid: interpolate error", func(t *testing.T) {
		got, err := NewConfigFromFile("", withTestReadFile, withTestInterpolateErr)

		assert.Nil(t, got)
		assert.Equal(t, "interpolate error", err.Error())
	})

	t.Run("invalid: read toml error", func(t *testing.T) {
		got, err := NewConfigFromFile("", withTestReadFile, withTestInterpolate, withTestReadTomlErr)

		assert.Nil(t, got)
		assert.Equal(t, "read toml error", err.Error())
	})
}
