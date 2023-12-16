package main

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"go/parser"
	"go/token"
	"log"
	"os"
	"path/filepath"

	openpolicy "github.com/4rchr4y/goray/analysis/rego"
	"github.com/4rchr4y/goray/ason"
	"github.com/open-policy-agent/opa/ast"
	"github.com/open-policy-agent/opa/rego"
	"github.com/open-policy-agent/opa/topdown"
	"github.com/spf13/cobra"
)

var rootCmd = &cobra.Command{
	Use:   "goray",
	Short: "",
	Long:  "",
	Run:   runOldCmd,
}

type failCase struct {
	Msg string `json:"msg"`
	Pos int    `json:"pos"`
	Sev string `json:"sev"`
}

type evalOutput struct {
	Fail []*failCase `json:"fail"`
}

var policies = []map[string]interface{}{
	{
		"Name":   "testName",
		"Source": "./analysis/policy/r1.rego",
	},
}

func runRootCmd(cmd *cobra.Command, args []string) {
	parsed := make(map[string]*ast.Module, len(policies))
	for _, p := range policies {
		file, err := os.Open(filepath.Clean(p["Source"].(string)))
		if err != nil {
			log.Fatal(err)
			return
		}

		policy, err := openpolicy.NewPolicy(file, p["Name"].(string))
		if err != nil {
			log.Fatal(err)
			return
		}

		module, err := openpolicy.ParseModule(policy)
		if err != nil {
			log.Fatal(err)
			return
		}

		parsed[p["Name"].(string)] = module
	}

	compiler, err := openpolicy.Compile(parsed, openpolicy.WithEnablePrintStatements(true))
	if err != nil {
		log.Fatal(err)
		return
	}

	fileMap, err := tmpGetFileAstAsMap("./ason/testdata/main.go")
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
