package compass

// type CompassOption func(*Compass)

// func DefaultCompass(options ...CompassOption) *Compass {
// 	pipeline := &pipe.Pipeline{}
// 	pipeline.Add(&plugin.CollectorPlugin{})

// 	compass := &Compass{
// 		pipeline: pipeline,
// 		config: &config.Config{
// 			RootDir:   ".",
// 			TargetDir: "",
// 			Analysis:  make(analyzer.AnalyzerMapOld),
// 		},

// 		eventCh:       make(chan event.Event, 1),
// 		unsubscribeCh: make(chan struct{}),
// 	}

// 	compass.RegisterAnalyzer(&analyzer.ImportAnalyzer{})
// 	compass.RegisterAnalyzer(&analyzer.StructAnalyzer{})
// 	compass.RegisterAnalyzer(&analyzer.FunctionAnalyzer{})

// 	for _, opt := range options {
// 		opt(compass)
// 	}

// 	return compass
// }

// func WithRootDir(dir string) CompassOption {
// 	return func(c *Compass) {
// 		c.config.RootDir = dir
// 	}
// }

// func WithTargetDir(dir string) CompassOption {
// 	return func(c *Compass) {
// 		c.config.TargetDir = dir
// 	}
// }
