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

	"github.com/4rchr4y/goray/analysis/openpolicy"
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

// var policies = []map[string]interface{}{
// 	{
// 		"Source": "opa/r1.rego",
// 		"Target": "testdata/main.go",
// 	},

// }

type Policy struct {
	Source       string
	Description  string
	Version      string
	Target       []string
	Dependencies map[string]string
}

var policies = []*Policy{
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
	std, err := openpolicy.DefineStdLib("opa/std")
	if err != nil {
		log.Fatal(err)
		return
	}

	parsed := make(map[string]*ast.Module, len(policies))
	for _, p := range policies {

		raw, err := os.ReadFile(p.Source)
		if err != nil {
			log.Fatal(err)
			return
		}

		module, err := openpolicy.ParseModule(p.Source, raw)
		if err != nil {
			log.Fatal(err)
			return
		}

		imports, err := processModuleImports(std, module)
		if err != nil {
			log.Fatal(err)
			return
		}

		parsed[p.Source] = module
		parsed = mergeMaps(parsed, imports)
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

func processModuleImports(std map[string]string, module *ast.Module) (map[string]*ast.Module, error) {
	if len(module.Imports) < 1 {
		return nil, nil
	}

	result := make(map[string]*ast.Module, len(module.Imports))

	for _, moduleImport := range module.Imports {

		importPath, ok := openpolicy.IsStdImport(std, moduleImport)
		if !ok {
			continue
		}

		raw, err := os.ReadFile(importPath)
		if err != nil {
			return nil, err
		}

		importModule, err := openpolicy.ParseModule(importPath, raw)
		if err != nil {
			return nil, err
		}

		result[importPath] = importModule

		imports, err := processModuleImports(std, importModule)
		if err != nil {
			return nil, err
		}

		if imports == nil {
			continue
		}

		result = mergeMaps(result, imports)
	}

	return result, nil
}

func mergeMaps(m1, m2 map[string]*ast.Module) map[string]*ast.Module {
	combined := make(map[string]*ast.Module, len(m1)+len(m2))
	for k, v := range m1 {
		combined[k] = v
	}

	for k, v := range m2 {
		combined[k] = v
	}

	return combined
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
