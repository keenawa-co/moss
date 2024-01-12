package bpm

type BundleFile struct {
	Package *PackageDef `toml:"package" validate:"required"`
}

type PackageDef struct {
	Name        string   `toml:"name" validate:"required"`
	Version     float64  `toml:"version" validate:"required"`
	Author      []string `toml:"author"`
	Description string   `toml:"description"`
}
