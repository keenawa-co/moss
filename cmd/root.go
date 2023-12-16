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
	"strings"

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

var policies = []map[string]interface{}{
	{
		"Source": "opa/r1.rego",
	},
}

func runRootCmd(cmd *cobra.Command, args []string) {
	parsed := make(map[string]*ast.Module, len(policies))
	for _, p := range policies {
		policyPath := filepath.Clean(p["Source"].(string))
		file, err := os.Open(policyPath)
		if err != nil {
			log.Fatal(err)
			return
		}

		policy, err := openpolicy.NewPolicy(file, policyPath)
		if err != nil {
			log.Fatal(err)
			return
		}

		module, err := openpolicy.ParseModule(policy)
		if err != nil {
			log.Fatal(err)
			return
		}

		parsed[policyPath] = module

		imports, err := processModuleImports(module)
		if err != nil {
			log.Fatal(err)
			return
		}

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

func processModuleImports(module *ast.Module) (map[string]*ast.Module, error) {
	if len(module.Imports) < 1 {
		return nil, nil
	}

	result := make(map[string]*ast.Module, len(module.Imports))

	for _, moduleImport := range module.Imports {
		importModulePath, ok := defineImportPath(moduleImport.Path.String())
		if !ok {
			continue
		}

		importFile, err := os.Open(importModulePath)
		if err != nil {
			return nil, err
		}

		importPolicy, err := openpolicy.NewPolicy(importFile, importModulePath)
		if err != nil {
			return nil, err
		}

		importModule, err := openpolicy.ParseModule(importPolicy)
		if err != nil {
			return nil, err
		}

		result[importModulePath] = importModule

		imports, err := processModuleImports(importModule)
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

func defineImportPath(moduleImport string) (string, bool) {
	if !strings.HasPrefix(moduleImport, "data") {
		return "", false
	}

	slashPath := strings.ReplaceAll(moduleImport, ".", "/")
	path := fmt.Sprintf("opa/%s.rego", strings.Clone(slashPath[5:]))
	return path, true
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
