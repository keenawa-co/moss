package config

import (
	"fmt"
	"os"
	"regexp"
	"strings"

	"github.com/BurntSushi/toml"
)

// func (cf *configFile) resolveEnvs() error {
// 	if cf == nil {
// 		return errors.New("'cf' can't be nil, should be a valid reference")
// 	}

// 	var err error
// 	if cf.Version, err = interpolate(os.LookupEnv, cf.Version); err != nil {
// 		return err
// 	}

// 	if cf.General.RootDir, err = interpolate(os.LookupEnv, cf.General.RootDir); err != nil {
// 		return err
// 	}

// 	if cf.General.TargetDir, err = interpolate(os.LookupEnv, cf.General.TargetDir); err != nil {
// 		return err
// 	}

// 	if cf.General.MaxParserConc, err = interpolate(os.LookupEnv, cf.General.MaxParserConc); err != nil {
// 		return err
// 	}

// 	if cf.General.MaxEngineConc, err = interpolate(os.LookupEnv, cf.General.MaxEngineConc); err != nil {
// 		return err
// 	}

// 	for i, v := range cf.General.IgnoreList {
// 		if cf.General.IgnoreList[i], err = interpolate(os.LookupEnv, v); err != nil {
// 			return err
// 		}
// 	}

// 	return nil
// }

// func newConfigFile(filePath string) (*configFile, error) {
// 	contents, err := os.ReadFile(filePath)
// 	if err != nil {
// 		return nil, err
// 	}

// 	cf := &configFile{
// 		General: &generalFile{},
// 	}

// 	if err := yaml.Unmarshal(contents, cf); err != nil {
// 		return nil, err
// 	}

// 	if err := cf.resolveEnvs(); err != nil {
// 		return nil, err
// 	}

// 	return cf, nil
// }

type configFileContentMap map[string]interface{}
type envLookupFunc func(key string) (string, bool)

const (
	fileContentsSize uint = 32
)

func getTomlFileContents(filePath string) (configFileContentMap, error) {
	data, err := os.ReadFile(filePath)
	if err != nil {
		return nil, err
	}

	contents := make(configFileContentMap, fileContentsSize)

	if err := toml.Unmarshal(data, &contents); err != nil {
		return nil, err
	}

	return contents, nil
}

func replaceEnvValuesMap(lookup envLookupFunc, content configFileContentMap) error {
	// for each value
	// interpolate value if the field is a string
	// make recursive call if the field is a map or an array
	// do nothing otherwise (only string values can potentially contain env variables)

	for key, val := range content {
		switch v := val.(type) {
		case string:
			newVal, err := interpolate(lookup, val.(string))
			if err != nil {
				return err
			}
			content[key] = newVal

		case configFileContentMap:
			if err := replaceEnvValuesMap(lookup, v); err != nil {
				return err
			}

		case []interface{}:
			if err := replaceEnvValuesSlice(lookup, v); err != nil {
				return err
			}
		}
	}

	return nil
}

func replaceEnvValuesSlice(lookup envLookupFunc, slice []interface{}) error {
	for key, val := range slice {
		switch v := val.(type) {
		case string:
			newVal, err := interpolate(lookup, val.(string))
			if err != nil {
				return err
			}
			slice[key] = newVal

		case configFileContentMap:
			if err := replaceEnvValuesMap(lookup, v); err != nil {
				return err
			}

		case []interface{}:
			if err := replaceEnvValuesSlice(lookup, v); err != nil {
				return err
			}
		}
	}

	return nil
}

// func hasEnv(strToCheck string) bool {
// 	return envVarPattern.MatchString(strToCheck)
// }

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
