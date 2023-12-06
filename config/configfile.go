package config

import (
	"os"

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

// Need to rethink the whole approach to env variables!!!
// REsolve envs within a file TEXT and then parse it to a msp!

// values should either be returned or modified by reference
func replaceEnvValues(lookup envLookupFunc, interpFunc interpolateFunc, value interface{}) error {
	switch v := value.(type) {
	case *string:

		// no need to do anything with empty string
		if len(*v) == 0 {
			return nil
		}

		// ToDo optional: check for env values and only call interpolate if env values are present
		newVal, err := interpFunc(lookup, *v)
		if err != nil {
			return err
		}

		*v = newVal

	case cfgMap:
		if err := replaceEnvValuesMap(lookup, interpFunc, v); err != nil {
			return err
		}

	case []string:
		if err := replaceEnvValuesSlice(lookup, interpFunc, v); err != nil {
			return err
		}

		// will make the test pass but is not general enough
		// case [][]string:
		// 	for i := range v {
		// 		if err := replaceEnvValues(lookup, interpFunc, v[i]); err != nil {
		// 			return err
		// 		}
		// 	}

	}

	return nil
}

func replaceEnvValuesMap(lookup envLookupFunc, interpFunc interpolateFunc, cm cfgMap) error {
	for i := range cm {
		if err := replaceEnvValues(lookup, interpFunc, cm[i]); err != nil {
			return err
		}
	}

	return nil
}

func replaceEnvValuesSlice(lookup envLookupFunc, interpFunc interpolateFunc, slice []string) error {
	for i := range slice {
		if err := replaceEnvValues(lookup, interpFunc, &slice[i]); err != nil {
			return err
		}
	}

	return nil
}
