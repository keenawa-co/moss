package rayfile

type Rayfile struct {
	Version   string `json:"version"`
	Workspace *workspace
}

type workspace struct {
	RootDir string `json:"root"`
}

type RayfileOptFn func(*Rayfile)

func New(options ...RayfileOptFn) *Rayfile {
	conf := &Rayfile{
		Version: defaultVersion,
		Workspace: &workspace{
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
