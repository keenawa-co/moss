package toml

import (
	"fmt"
	"regexp"
	"strings"

	"github.com/4rchr4y/goray/internal/syswrap"
	"github.com/BurntSushi/toml"
)

type osWrapper interface {
	LookupEnv(key string) (string, bool)
}

type decoderClient interface {
	Decode(data string, v interface{}) (toml.MetaData, error)
}

type TomlService struct {
	os      osWrapper
	decoder decoderClient
}

func NewTomlService() *TomlService {
	return &TomlService{
		os:      new(syswrap.OsClient),
		decoder: new(Decoder),
	}
}

const envVarString = `\$\{[A-Za-z_][A-Za-z0-9_]*\}`

var envVarPattern = regexp.MustCompile(envVarString)

func (ts *TomlService) Decode(data string, value interface{}) error {
	content, err := ts.interpolate(data)
	if err != nil {
		return err
	}

	if _, err := ts.decoder.Decode(content, value); err != nil {
		return err
	}

	return nil
}

func (ts *TomlService) interpolate(data string) (string, error) {
	// preliminary check for the presence of placeholders
	if !strings.Contains(data, "${") {
		return data, nil
	}

	var missingVars []string
	result := envVarPattern.ReplaceAllStringFunc(data, func(match string) string {
		envKey := strings.Clone(match[2 : len(match)-1])
		if value, exists := ts.os.LookupEnv(envKey); exists {
			return value
		}

		missingVars = append(missingVars, envKey)

		return match
	})

	// check if there are any unresolved variables
	if len(missingVars) > 0 {
		return "", fmt.Errorf("environment variables not found: %s", strings.Join(missingVars, ", "))
	}

	return result, nil
}
