package compass

var DefaultIgnoredList = map[string]struct{}{
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

type Config struct {
	RootDir     string
	TargetDir   string
	IgnoredList map[string]struct{}

	MaxParserConcurrency uint
	MaxEngineConcurrency uint

	Group PickerFactoryGroup
}
