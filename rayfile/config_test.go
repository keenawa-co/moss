package rayfile

import (
	"errors"
	"testing"

	"github.com/BurntSushi/toml"
	"github.com/stretchr/testify/assert"
)

const configFileText = `
	[workspace]
	version = "1.0.193"
	root= "."
	policies="./policy"
	ignore-list=[".git", ".docker"]
	go-arch="HOME: ${GORAY}"

	[analysis]
	KRAIL1001 = {level = "info", target= "./${GORAY}"}

	[analysis.SE]
	level = "critical"
	target = "./internal"
`

var mockEnvVariables = map[string]string{
	"GORAY":         "some_value",
	"THREADS_COUNT": "5",
}

func mockLookupEnv(key string) (string, bool) {
	value, exists := mockEnvVariables[key]
	return value, exists
}

func TestInterpolate(t *testing.T) {
	t.Run("valid: one env variable found", func(t *testing.T) {
		got, err := interpolate(mockLookupEnv, "${GORAY}")

		assert.Equal(t, "some_value", got)
		assert.Nil(t, err)
	})

	t.Run("valid: one env variable found within the same string", func(t *testing.T) {
		got, err := interpolate(mockLookupEnv, "abc ${GORAY} abc")

		assert.Equal(t, "abc some_value abc", got)
		assert.Nil(t, err)
	})

	t.Run("valid: two env variables found within the same string", func(t *testing.T) {
		got, err := interpolate(mockLookupEnv, "${GORAY}${THREADS_COUNT}")

		assert.Equal(t, "some_value5", got)
		assert.Nil(t, err)
	})

	t.Run("valid: two env variables found within the same string", func(t *testing.T) {
		got, err := interpolate(mockLookupEnv, "${GORAY}${THREADS_COUNT}")

		assert.Equal(t, "some_value5", got)
		assert.Nil(t, err)
	})

	t.Run("valid: no env variables within the string", func(t *testing.T) {
		got, err := interpolate(mockLookupEnv, "${NOT AN ENV")

		assert.Equal(t, "${NOT AN ENV", got)
		assert.Nil(t, err)
	})

	t.Run("invalid: env variable not found", func(t *testing.T) {
		got, err := interpolate(mockLookupEnv, "${SOME_NAME}")

		assert.Equal(t, "", got)
		assert.Equal(t, "environment variables not found: SOME_NAME", err.Error())
	})

	t.Run("invalid: one env variable found, one not found", func(t *testing.T) {
		got, err := interpolate(mockLookupEnv, "${GORAY}${SOME_NAME}")

		assert.Equal(t, "", got)
		assert.Equal(t, "environment variables not found: SOME_NAME", err.Error())
	})

	t.Run("invalid: multiple env variables not found", func(t *testing.T) {
		got, err := interpolate(mockLookupEnv, "${SOME_NAME1} ${SOME_NAME2} ${SOME_NAME3}")

		assert.Equal(t, "", got)
		assert.Equal(t, "environment variables not found: SOME_NAME1, SOME_NAME2, SOME_NAME3", err.Error())
	})
}

func withMockReadFile(rc *ReadConf) {
	rc.readFile = mockReadFile
}

func mockReadFile(name string) ([]byte, error) {
	return []byte(configFileText), nil
}

func withMockReadFileErr(rc *ReadConf) {
	rc.readFile = mockReadFileErr
}

func mockReadFileErr(name string) ([]byte, error) {
	return nil, errors.New("read file error")
}

func withMockReadToml(rc *ReadConf) {
	rc.readToml = mockReadToml
}

func mockReadToml(data string, v interface{}) (toml.MetaData, error) {
	return toml.MetaData{}, nil
}

func withMockReadTomlErr(rc *ReadConf) {
	rc.readToml = mockReadTomlErr
}

func mockReadTomlErr(data string, v interface{}) (toml.MetaData, error) {
	return toml.MetaData{}, errors.New("read toml error")

}

func withMockInterpolate(rc *ReadConf) {
	rc.interpolate = mockInterpolate
}

func mockInterpolate(lookup LookupFn, strWithEnvs string) (string, error) {
	return "interpolated string", nil
}

func withMockInterpolateErr(rc *ReadConf) {
	rc.interpolate = mockInterpolateErr
}

func mockInterpolateErr(lookup LookupFn, strWithEnvs string) (string, error) {
	return "", errors.New("interpolate error")
}

func withMockLookup(rc *ReadConf) {
	rc.lookup = mockLookup
}

func mockLookup(key string) (string, bool) {
	return "", false
}

func withMockLookupEnv(rc *ReadConf) {
	rc.lookup = mockLookupEnv
}

func TestNewConfigFromFile(t *testing.T) {

	t.Run("valid: default config is returned", func(t *testing.T) {
		got, err := NewConfigFromFile("", withMockReadFile, withMockInterpolate, withMockReadToml, withMockLookup)

		expected := NewConfig()
		assert.Equal(t, expected, got)
		assert.Nil(t, err)
	})

	t.Run("valid: content gets parsed and interpolated correctly", func(t *testing.T) {
		got, err := NewConfigFromFile("", withMockReadFile, withMockLookupEnv)

		expected := NewConfig()
		expected.Workspace.Version = "1.0.193"
		expected.Workspace.RootDir = "."
		expected.Workspace.PolicyDir = "./policy"

		expected.Workspace.GoArch = "HOME: some_value"

		analysis := make(map[string]*AnalysisConf)
		analysis["KRAIL1001"] = &AnalysisConf{
			Level:  "info",
			Target: "./some_value",
		}

		analysis["SE"] = &AnalysisConf{
			Level:  "critical",
			Target: "./internal",
		}

		expected.Analysis = analysis

		expected.Workspace.IgnoreList = []string{".git", ".docker"}

		assert.Equal(t, expected, got)
		assert.Nil(t, err)
	})

	t.Run("invalid: file read error", func(t *testing.T) {
		got, err := NewConfigFromFile("", withMockReadFileErr)

		assert.Nil(t, nil, got)
		assert.Equal(t, "read file error", err.Error())
	})

	t.Run("invalid: interpolate error", func(t *testing.T) {
		got, err := NewConfigFromFile("", withMockReadFile, withMockInterpolateErr)

		assert.Nil(t, got)
		assert.Equal(t, "interpolate error", err.Error())
	})

	t.Run("invalid: read toml error", func(t *testing.T) {
		got, err := NewConfigFromFile("", withMockReadFile, withMockInterpolate, withMockReadTomlErr)

		assert.Nil(t, got)
		assert.Equal(t, "read toml error", err.Error())
	})
}

func TestNewConfig(t *testing.T) {

	t.Run("all default values are applied", func(t *testing.T) {
		got := NewConfig()

		expected := &Config{
			Workspace: &workspace{
				Version:    defaultVersion,
				RootDir:    defaultRoot,
				PolicyDir:  defaultPolicies,
				GoArch:     defaultGoArch,
				IgnoreList: defaultIgnoreList,
			},
		}

		assert.Equal(t, expected, got)
	})
}
