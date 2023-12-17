package config

type Policy struct {
	Source       string
	Description  string
	Version      string
	Target       []string
	Dependencies map[string]string
}
