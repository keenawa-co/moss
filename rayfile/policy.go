package rayfile

type Def interface{}

type PolicyDef struct {
	Path         string
	Target       []string
	Dependencies []string
}
