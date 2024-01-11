package rpm

type JudgeFile struct {
	Package *PackageDef
}

type PackageDef struct {
	Name        string
	Version     string
	Author      []string
	Description string
	Keywords    []string
}
