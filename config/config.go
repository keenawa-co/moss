package config

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

// func NewFromFile(filePath string) (*Config, error) {

// cfg := newDefaultConfig()

// cfg, err := newDefaultConfig()
// if err != nil {
// 	return nil, err
// }

// if err := cfg.validate(); err != nil {
// 	return nil, err
// }

// return cfg, nil
// }

func Unmarshal(buffer []byte, cfg *Config) {

}

// func (cfg *Config) validate() error {
// 	if cfg.Version == defaultVersion {
// 		return errors.New("version field must be set")
// 	}

// 	if _, versionAvailable := availableVersions[cfg.Version]; !versionAvailable {
// 		return errors.New("specified version is not available")
// 	}

// 	return nil
// }

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
