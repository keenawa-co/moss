package rego

type Policy struct {
	_        [0]int
	Name     string
	Source   string
	Raw      []byte
	Target   string
	Query    string
	Requires []*Policy
}
