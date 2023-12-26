package rayfile

import (
	"fmt"
	"os"
	"regexp"
	"strings"

	"github.com/BurntSushi/toml"
)

// ToDos:
// Implement Validate function

type LookupFn func(key string) (string, bool)

type Config struct {
	Workspace *workspace               `toml:"workspace"`
	Analysis  map[string]*AnalysisConf `toml:"analysis"`
}

type workspace struct {
	Version    string   `toml:"version"`
	RootDir    string   `toml:"root"`
	PolicyDir  string   `toml:"policies"`
	IgnoreList []string `toml:"ignore-list"`
	GoArch     string   `toml:"go-arch"`
}

type AnalysisConf struct {
	Level  string
	Target string
}

type ConfOptFn func(*Config)

func NewConfig(options ...ConfOptFn) *Config {
	conf := &Config{
		Workspace: &workspace{
			Version:    defaultVersion,
			RootDir:    defaultRoot,
			PolicyDir:  defaultPolicies,
			GoArch:     defaultGoArch,
			IgnoreList: defaultIgnoreList,
		},
	}

	for i := 0; i < len(options); i++ {
		options[i](conf)
	}

	return conf
}

func WithRootDir(dirPath string) ConfOptFn {
	return func(c *Config) {
		c.Workspace.RootDir = dirPath
	}
}

func WithPolicyDir(dirPath string) ConfOptFn {
	return func(c *Config) {
		c.Workspace.PolicyDir = dirPath
	}
}

func WithGoArch(goArch string) ConfOptFn {
	return func(c *Config) {
		c.Workspace.GoArch = goArch
	}
}

func WithAnalysis(analysis map[string]*AnalysisConf) ConfOptFn {
	return func(c *Config) {
		c.Analysis = analysis
	}
}

type ConfReadFileOptFn func(*ReadConf)

type ReadConf struct {
	lookup      func(key string) (string, bool)
	readFile    func(name string) ([]byte, error)
	readToml    func(data string, v interface{}) (toml.MetaData, error)
	interpolate func(lookup LookupFn, strWithEnvs string) (string, error)
}

func NewFromFile(options ...ConfReadFileOptFn) *ReadConf {
	readConf := &ReadConf{
		lookup:      os.LookupEnv,
		readFile:    os.ReadFile,
		readToml:    toml.Decode,
		interpolate: interpolate,
	}

	for i := 0; i < len(options); i++ {
		options[i](readConf)
	}

	return readConf
}

func NewConfigFromFile(filePath string, options ...ConfReadFileOptFn) (*Config, error) {
	readConf := NewFromFile(options...)

	conf := NewConfig()

	data, err := readConf.readFile(filePath)
	if err != nil {
		return nil, err
	}

	dataWithEnv, err := readConf.interpolate(readConf.lookup, string(data))
	if err != nil {
		return nil, err
	}

	if _, err := readConf.readToml(dataWithEnv, &conf); err != nil {
		return nil, err
	}

	return conf, nil
}

func (cfg *Config) Validate() error {
	return nil
}

const (
	envVarString = `\$\{[A-Za-z_][A-Za-z0-9_]*\}`
)

var (
	envVarPattern = regexp.MustCompile(envVarString)
)

func interpolate(lookup LookupFn, strWithEnvs string) (string, error) {
	missingVariables := make([]string, 0)
	resultStr := envVarPattern.ReplaceAllStringFunc(strWithEnvs, func(match string) string {
		envKey := strings.Clone(match[2 : len(match)-1])
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
