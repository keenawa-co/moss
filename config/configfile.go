package config

import (
	"fmt"
	"os"
	"regexp"
	"strings"

	"github.com/BurntSushi/toml"
)

type cfgMap map[string]interface{}
type envLookupFunc func(key string) (string, bool)
type interpolateFunc func(lookup envLookupFunc, strWithEnvs string) (string, error)

const (
	fileContentsSize uint = 32
)

func getTomlFileContents(filePath string) (cfgMap, error) {
	data, err := os.ReadFile(filePath)
	if err != nil {
		return nil, err
	}

	contents := make(cfgMap, fileContentsSize)

	if err := toml.Unmarshal(data, &contents); err != nil {
		return nil, err
	}

	return contents, nil
}

// values should either be returned or modified by reference
func replaceEnvValues(lookup envLookupFunc, interpFunc interpolateFunc, value interface{}) (interface{}, error) {
	switch v := value.(type) {
	case string:
		if len(v) == 0 {
			return v, nil
		}

		// ToDo optional: check for env values and only call interpolate if env values are present
		newVal, err := interpFunc(lookup, v)
		if err != nil {
			return nil, err
		}
		return newVal, nil

	case cfgMap:
		if err := replaceEnvValuesMap(lookup, interpFunc, v); err != nil {
			return nil, err
		}
		return v, nil

	case []string:
		if err := replaceEnvValuesSlice(lookup, interpFunc, v); err != nil {
			return nil, err
		}
		return v, nil
	}

	return value, nil
}

func replaceEnvValuesMap(lookup envLookupFunc, interpFunc interpolateFunc, content cfgMap) error {
	for key, val := range content {
		newVal, err := replaceEnvValues(lookup, interpFunc, val)
		if err != nil {
			return err
		}
		content[key] = newVal
	}

	return nil
}

func replaceEnvValuesSlice(lookup envLookupFunc, interpFunc interpolateFunc, slice []string) error {
	for key, val := range slice {
		newVal, err := replaceEnvValues(lookup, interpFunc, val)
		if err != nil {
			return err
		}
		slice[key] = newVal.(string)
	}

	return nil
}

const (
	envVarString = `\$\{[A-Za-z_][A-Za-z0-9_]*\}`
)

var (
	envVarPattern = regexp.MustCompile(envVarString)
)

func interpolate(lookup envLookupFunc, strWithEnvs string) (string, error) {
	missingVariables := make([]string, 0)

	resultStr := envVarPattern.ReplaceAllStringFunc(strWithEnvs, func(match string) string {
		envKey := match[2 : len(match)-1]
		if value, exists := lookup(envKey); exists {
			return value
		}

		missingVariables = append(missingVariables, envKey)

		return match
	})

	// check if there are any unresolved variables
	if len(missingVariables) > 0 {
		return "", fmt.Errorf("environment variables not found: %s", strings.Join(missingVariables, ", "))
	}

	return resultStr, nil
}
