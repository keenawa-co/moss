package main

import (
	"fmt"

	"golang.org/x/tools/go/analysis"
)

var Analyzer = analysis.Analyzer{
	Name: "globalvar",
	Doc:  "reports global variables usage",
	Run:  run,
}

func run(pass *analysis.Pass) (interface{}, error) {
	fmt.Println("------", pass.Pkg.Name())
	for _, v := range pass.TypesInfo.Defs {
		if v != nil {
			fmt.Println(v.Type(), v.Name())
		}

	}

	return nil, nil
}

func main() {}
