package collector

import "github.com/4rchr4y/go-compass/config"

type CollectorOption func(*Collector)

func WithIgnoredList(list ...string) CollectorOption {
	return func(c *Collector) {
		for _, item := range list {
			c.ignoredList[item] = struct{}{}
		}
	}
}

func WithTargetDir(dir string) CollectorOption {
	return func(c *Collector) {
		c.targetDir = dir
	}
}

func WithRootDir(dir string) CollectorOption {
	return func(c *Collector) {
		c.rootDir = dir
	}
}

func DefaultCollector(options ...CollectorOption) *Collector {
	collector := &Collector{
		ignoredList: config.DefaultIgnoredMap,
		rootDir:     ".",
		targetDir:   "",
		info:        &Info{},
		packageDirs: make([]string, 0),
	}

	for _, opt := range options {
		opt(collector)
	}

	return collector
}
