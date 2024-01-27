package rayfile

type Rayfile struct {
	Version   string        `toml:"version"`
	Workspace *WorkspaceDef `toml:"workspace"`
}

type WorkspaceDef struct {
	RootDir string `toml:"root"`
}

type RayfileOptFn func(*Rayfile)

func New(options ...RayfileOptFn) *Rayfile {
	conf := &Rayfile{
		Version: defaultVersion,
		Workspace: &WorkspaceDef{
			RootDir: defaultRoot,
		},
	}

	for i := 0; i < len(options); i++ {
		options[i](conf)
	}

	return conf
}

func WithRootDir(dirPath string) RayfileOptFn {
	return func(c *Rayfile) {
		c.Workspace.RootDir = dirPath
	}
}
