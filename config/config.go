package config

import (
	"github.com/mitchellh/mapstructure"
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
	Workspace *workspace                  `mapstructure:"workspace"`
	Analysis  map[string]analysisSettings `mapstructure:"analysis"`
}

type workspace struct {
	Version    string   `mapstructure:"version"`
	Root       string   `mapstructure:"root"`
	Policies   string   `mapstructure:"policies"`
	IngoreList []string `mapstructure:"ignore-list"`
	GoArch     string   `mapstructure:"go-arch"`
}

type analysisSettings struct {
	Level  string
	Target string
}

func NewFromFile(filePath string) (*Config, error) {

	cfg := newDefaultConfig()

	cfgMap, err := getTomlFileContents(filePath)
	if err != nil {
		return nil, err
	}

	mapstructure.Decode(cfgMap, cfg)

	if err := cfg.validate(); err != nil {
		return nil, err
	}

	return cfg, nil
}

// func copyContentMapToConfig(cfgMap configFileContentMap, cfg *Config) error {
// 	return mapstructure.Decode(cfgMap, cfg)
// }

func Unmarshal(buffer []byte, cfg *Config) {

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
