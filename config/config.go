package config

import (
	"fmt"
	"os"
	"regexp"
	"strings"

	"github.com/BurntSushi/toml"
)

// Strings
const (
	defaultVersion  = ""
	defaultRoot     = "./"
	defaultPolicies = "./"
	defaultGoArch   = "${GOARCH}"
)

// Integers
const ()

var availableVersions = map[string]struct{}{
	"1.0": {},
	"1.1": {},
}

type Config struct {
	Workspace *workspace                  `toml:"workspace"`
	Analysis  map[string]analysisSettings `toml:"analysis"`
}

type workspace struct {
	Version    string   `toml:"version"`
	Root       string   `toml:"root"`
	Policies   string   `toml:"policies"`
	IngoreList []string `toml:"ignore-list"`
	GoArch     string   `toml:"go-arch"`
}

type analysisSettings struct {
	Level  string
	Target string
}

func NewFromFile(filePath string) (*Config, error) {

	cfg := newDefaultConfig()

	if err := decodeWithEnvsFromFile(os.LookupEnv, interpolate, filePath, cfg); err != nil {
		return nil, err
	}

	// mapstructure.Decode(cfgMap, cfg)

	if err := cfg.validate(); err != nil {
		return nil, err
	}

	return cfg, nil
}

func (cfg *Config) validate() error {
	// if cfg.Version == defaultVersion {
	// 	return errors.New("version field must be set")
	// }

	// if _, versionAvailable := availableVersions[cfg.Version]; !versionAvailable {
	// 	return errors.New("specified version is not available")
	// }

	// ToDo: implement validate function

	return nil
}

func newDefaultConfig() *Config {
	ws := workspace{
		Version:    defaultVersion,
		Root:       defaultRoot,
		Policies:   defaultPolicies,
		IngoreList: nil,
		GoArch:     defaultGoArch,
	}

	return &Config{
		Workspace: &ws,
	}
}

const (
	envVarString = `\$\{[A-Za-z_][A-Za-z0-9_]*\}`
)

var (
	envVarPattern = regexp.MustCompile(envVarString)
)

type envLookupFunc func(key string) (string, bool)
type interpolateFunc func(lookup envLookupFunc, strWithEnvs string) (string, error)

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

func decodeWithEnvsFromFile(lookup envLookupFunc, interpolate interpolateFunc, filePath string, cfg *Config) error {
	data, err := readFile(filePath)
	if err != nil {
		return err
	}

	dataWithEnv, err := interpolate(lookup, data)
	if err != nil {
		return err
	}

	if _, err := toml.Decode(dataWithEnv, &cfg); err != nil {
		return err
	}

	return nil
}

func readFile(filePath string) (string, error) {
	data, err := os.ReadFile(filePath)
	if err != nil {
		return "", err
	}

	return string(data), nil
}
