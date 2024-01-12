package bpm

type BundleLock struct {
	Version int          `toml:"version"`
	Modules []*ModuleDef `toml:"modules"`
}

type ModuleDef struct {
	Name         string   `toml:"name"`
	Source       string   `toml:"source"`
	Checksum     string   `toml:"checksum"`
	Dependencies []string `toml:"dependencies"`
}
