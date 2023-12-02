// Copyright 2023 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

// Package appends defines an Analyzer that detects
// if there is only one variable in append.
package plugin

import (
	"fmt"
	"plugin"

	"golang.org/x/tools/go/analysis"
)

type failCase struct {
	Msg string `json:"msg"`
	Pos int    `json:"pos"`
	Sev string `json:"sev"`
}

const df = 10

var SomeConst = failCase{
	Msg: "reg",
}

/* package scope (this file only) */
func LoadPlugin(path string) (*analysis.Analyzer, error) {

	fmt.Println("smt")
	fmt.Sprintln("smt")
	p, err := plugin.Open(path)
	if err != nil {
		return nil, err
	}

	symbol, err := p.Lookup("Analyzer")
	if err != nil {
		return nil, err
	}

	analyzer, ok := symbol.(*analysis.Analyzer)
	if !ok {
		return nil, fmt.Errorf("symbol 'Analyzer' %s is not a type *analysis.Analyzer", path)
	}

	fmt.Println("smt")

	return analyzer, nil
}
