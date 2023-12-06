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

func getCfgMapWithEnvsFromFile(lookup envLookupFunc, interpolate interpolateFunc, filePath string) (cfgMap, error) {
	data, err := readFile(filePath)
	if err != nil {
		return nil, err
	}

	dataWithEnv, err := interpolate(lookup, data)
	if err != nil {
		return nil, err
	}
	contents := make(cfgMap, fileContentsSize)

	if _, err := toml.Decode(dataWithEnv, &contents); err != nil {
		return nil, err
	}

	return contents, nil
}

func readFile(filePath string) (string, error) {
	data, err := os.ReadFile(filePath)
	if err != nil {
		return "", err
	}

	return string(data), nil
}
