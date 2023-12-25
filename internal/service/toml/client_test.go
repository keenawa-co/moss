package toml

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

const testTomlData = `
user = "${USER}"
[owner]
terminal = "${TERM}"
`

var testEnv = map[string]string{
	"USER": "g10z3r",
	"TERM": "xterm-256color",
}

type mockOsWrapper struct {
	env map[string]string
}

func (m *mockOsWrapper) LookupEnv(key string) (string, bool) {
	value, exists := m.env[key]
	return value, exists
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

	t.Run("String with multiple environment variables", func(t *testing.T) {
		input := "Path is ${EXISTING_VAR}/bin:${MISSING_VAR}/bin"
		_, err := service.interpolate(input)
		assert.Error(t, err, "An error should occur when at least one variable is missing")
		if err != nil {
			assert.Contains(t, err.Error(), "MISSING_VAR", "The error message should contain the name of the missing variable")
		}
	})
}
