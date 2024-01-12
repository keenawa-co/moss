package toml

import (
	"fmt"
	"io"
	"regexp"
	"strings"

	"github.com/4rchr4y/goray/internal/infra/syswrap"
	"github.com/BurntSushi/toml"
)

type osClient interface {
	LookupEnv(key string) (string, bool)
}

type decoder interface {
	Decode(data string, v interface{}) (toml.MetaData, error)
}

type encoder interface {
	Encode(w io.Writer, v interface{}) error
}

type TomlService struct {
	os  osClient
	dec decoder
	en  encoder
}

func NewTomlService() *TomlService {
	return &TomlService{
		os:  new(syswrap.OsClient),
		dec: new(Decoder),
		en:  new(Encoder),
	}
}

const envVarString = `\$\{[A-Za-z_][A-Za-z0-9_]*\}`

var envVarPattern = regexp.MustCompile(envVarString)

func (ts *TomlService) Encode(writer io.Writer, value interface{}) error {
	return ts.en.Encode(writer, value)
}

func (ts *TomlService) Decode(data string, value interface{}) error {
	content, err := ts.interpolate(data)
	if err != nil {
		return err
	}

	if _, err := ts.dec.Decode(content, value); err != nil {
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
