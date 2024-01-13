package bpm

type BpmWorkFile struct {
	Workspace *WorkspaceDef `toml:"workspace"`
}

type WorkspaceDef struct {
	Path     string   `toml:"path"`
	Author   []string `toml:"author"`
	Packages []string `toml:"packages"`
}
