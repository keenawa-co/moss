package main

import (
	"fmt"

	"golang.org/x/tools/go/analysis"
)

var Analyzer = analysis.Analyzer{
	Name: "globalvar2",
	Doc:  "reports global variables usage",
	Run:  run,
}

func run(pass *analysis.Pass) (interface{}, error) {
	fmt.Println("test ")
	return nil, nil
}

func main() {}
