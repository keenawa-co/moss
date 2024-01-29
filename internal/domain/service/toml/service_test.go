package toml

import (
	"errors"
	"fmt"
	"testing"

	"github.com/BurntSushi/toml"
	"github.com/stretchr/testify/assert"
)

const (
	testCorrectTomlData   = `user = "${USER}"`
	testIncorrectTomlData = `user = "${MISSING_ENV_VAR}"`
	testInvalidTomlData   = `Invalid TOML file`
)

var (
	testEnv = map[string]string{
		"USER": "g10z3r",
		"TERM": "xterm-256color",
	}

	testDecoderResp = map[string]interface{}{
		"user": testEnv["USER"],
	}
)

type mockOsWrapper struct {
	env map[string]string
}

func (m *mockOsWrapper) LookupEnv(key string) (string, bool) {
	value, exists := m.env[key]
	return value, exists
}

type mockDecoderClient struct {
	resp map[string]interface{}
	err  error
}

func (m *mockDecoderClient) Decode(data string, v interface{}) (toml.MetaData, error) {
	v = m.resp
	return toml.MetaData{}, m.err
}

func TestDecode(t *testing.T) {
	t.Run("Successful decode", func(t *testing.T) {
		service := TomlService{
			os: &mockOsWrapper{env: testEnv},
			dec: &mockDecoderClient{
				resp: testDecoderResp,
			},
		}

		var result map[string]interface{}
		err := service.Decode(testCorrectTomlData, &result)
		assert.NoError(t, err, "Decode should succeed without error")
	})

	t.Run("Interpolation error", func(t *testing.T) {
		service := TomlService{
			os: &mockOsWrapper{env: make(map[string]string)},
			dec: &mockDecoderClient{
				resp: testDecoderResp,
			},
		}

		err := service.Decode(testIncorrectTomlData, &map[string]interface{}{})
		assert.Error(t, err, "Decode should fail due to missing environment variable")
	})

	t.Run("Decoding error", func(t *testing.T) {
		service := TomlService{
			os: &mockOsWrapper{env: testEnv},
			dec: &mockDecoderClient{
				err: errors.New("decoding error"),
			},
		}

		err := service.Decode(testInvalidTomlData, &map[string]interface{}{})
		assert.Error(t, err, "Decode should fail due to decoding error")
	})
}

func TestInterpolate(t *testing.T) {
	service := TomlService{os: &mockOsWrapper{env: testEnv}}

	t.Run("String without placeholders", func(t *testing.T) {
		input := "This is a test string."
		expected := "This is a test string."
		result, err := service.interpolate(input)
		assert.NoError(t, err, "No error should occur for string without placeholders")
		assert.Equal(t, expected, result, "The input string should be returned as is")
	})

	t.Run("String with existing environment variable", func(t *testing.T) {
		input := "Path is ${USER}/bin"
		expected := fmt.Sprintf("Path is %s/bin", testEnv["USER"])
		result, err := service.interpolate(input)
		assert.NoError(t, err, "No error should occur for existing environment variable")
		assert.Equal(t, expected, result, "The environment variable should be replaced correctly")
	})

	t.Run("String with missing environment variable", func(t *testing.T) {
		input := "Path is ${MISSING_VAR}/bin"
		_, err := service.interpolate(input)
		assert.Error(t, err, "An error should occur for missing environment variable")
		if err != nil {
			assert.Contains(t, err.Error(), "MISSING_VAR", "The error message should contain the name of the missing variable")
		}
	})

	t.Run("String with existing and missing environment variables", func(t *testing.T) {
		input := "Path is ${EXISTING_VAR}/bin:${MISSING_VAR}/bin"
		_, err := service.interpolate(input)
		assert.Error(t, err, "An error should occur when at least one variable is missing")
		if err != nil {
			assert.Contains(t, err.Error(), "MISSING_VAR", "The error message should contain the name of the missing variable")
		}
	})

	t.Run("String with multiple existing environment variable", func(t *testing.T) {
		input := "Path is ${USER}/bin/${TERM}"
		expected := fmt.Sprintf("Path is %s/bin/%s", testEnv["USER"], testEnv["TERM"])
		result, err := service.interpolate(input)
		assert.NoError(t, err, "No error should occur for existing environment variables")
		assert.Equal(t, expected, result, "All environment variables should be replaced correctly")
	})

	t.Run("String with multiple non-existing environment variables", func(t *testing.T) {
		input := "Path is ${MISSING_VAR1}/bin:${MISSING_VAR2}/bin"
		_, err := service.interpolate(input)
		assert.Error(t, err, "An error should occur when at least one variable is missing")
		if err != nil {
			assert.Contains(t, err.Error(), "MISSING_VAR1", "The error message should contain the name of the missing variable")
			assert.Contains(t, err.Error(), "MISSING_VAR2", "The error message should contain the name of the missing variable")
		}
	})

	t.Run("String with misspelled placeholder", func(t *testing.T) {
		input := "Path is ${USER/bin"
		expected := "Path is ${USER/bin"
		result, err := service.interpolate(input)
		assert.NoError(t, err, "No error should occur for string with misspelled placeholder")
		assert.Equal(t, expected, result, "The input string should be returned as is")
	})
}
