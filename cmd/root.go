package main

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"go/parser"
	"go/token"
	"log"

	"github.com/4rchr4y/goray/analysis/openpolicy"
	"github.com/4rchr4y/goray/ason"
	"github.com/4rchr4y/goray/config"
	"github.com/open-policy-agent/opa/rego"
	"github.com/open-policy-agent/opa/topdown"
	"github.com/spf13/cobra"
)

var rootCmd = &cobra.Command{
	Use:   "goray",
	Short: "",
	Long:  "",
	Run:   runRootCmd,
}

type failCase struct {
	Msg string `json:"msg"`
	Pos int    `json:"pos"`
	Sev string `json:"sev"`
}

type evalOutput struct {
	Fail []*failCase `json:"fail"`
}

var policies = []*config.Policy{
	{
		Source:      "opa/r1.rego",
		Description: "some short description here",
		Version:     "0.0.1",
		Target:      []string{"testdata/main.go"},
		Dependencies: map[string]string{
			"go.ast.types": "opa/go/ast/types",
		},
	},
}

func runRootCmd(cmd *cobra.Command, args []string) {
	regoCli, err := openpolicy.NewRegoClient("opa/std")
	if err != nil {
		log.Fatal(err)
		return
	}

	moduleList, err := openpolicy.NewModuleList(regoCli, policies)
	if err != nil {
		log.Fatal(err)
		return
	}

	merged := openpolicy.MergeList(moduleList)

	compiler, err := openpolicy.Compile(merged, openpolicy.WithEnablePrintStatements(true))
	if err != nil {
		log.Fatal(err)
		return
	}

	fileMap, err := tmpGetFileAstAsMap("testdata/main.go")
	if err != nil {
		log.Fatal(err)
		return
	}

	var buf bytes.Buffer
	r := rego.New(
		rego.Query("data.goray"),
		rego.Compiler(compiler),
		rego.Input(fileMap),
		rego.EnablePrintStatements(true),
		rego.PrintHook(topdown.NewPrintHook(&buf)),
	)

	rs, err := r.Eval(context.Background())
	if err != nil {
		log.Fatal(err)
		return
	}

	for _, result := range rs {
		for _, r := range result.Expressions {
			fmt.Println(r.Value)
		}
	}

	fmt.Println(buf.String())
}

func tmpGetFileAstAsMap(filePath string) (map[string]interface{}, error) {
	fset := token.NewFileSet()

	f, err := parser.ParseFile(fset, filePath, nil, parser.ParseComments)
	if err != nil {
		return nil, err
	}

	pass := ason.NewSerPass(fset)
	fileAstJson := ason.SerializeFile(pass, f)

	jsonData, err := json.Marshal(fileAstJson)
	if err != nil {
		return nil, err
	}

	var fileMap map[string]interface{}
	if err := json.Unmarshal(jsonData, &fileMap); err != nil {
		return nil, err
	}

	return fileMap, nil
}
