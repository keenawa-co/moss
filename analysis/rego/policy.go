package rego

type Policy struct {
	_        [0]int
	Name     string
	Source   string
	Target   string
	Requires []*Policy
}
