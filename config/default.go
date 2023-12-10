package config

const (
	defaultVersion  = ""
	defaultRoot     = "./"
	defaultPolicies = "./"
	defaultGoArch   = "${GOARCH}"
)

var defaultIgnoredList = map[string]struct{}{
	".git":    {},
	".docker": {},

	".vscode":  {},
	".idea":    {},
	".eclipse": {},

	"dist":    {},
	"docker":  {},
	"assets":  {},
	"vendor":  {},
	"build":   {},
	"scripts": {},
	"ci":      {},
	"log":     {},
	"logs":    {},
}

var availableVersions = map[string]struct{}{
	"1.0": {},
	"1.1": {},
}
