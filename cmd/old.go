package main

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"go/ast"
	"go/parser"
	"go/printer"
	"go/token"
	"log"
	"os"
	"path/filepath"

	"github.com/4rchr4y/goray/ason"
	regoAst "github.com/open-policy-agent/opa/ast"
	"github.com/open-policy-agent/opa/rego"
	"github.com/open-policy-agent/opa/topdown"
	"github.com/spf13/cobra"
)

var oldCmd = &cobra.Command{
	Use:   "old",
	Short: "",
	Long:  "",
	Run:   runOldCmd,
}

func init() {
	rootCmd.AddCommand(oldCmd)

}

func evaluateRegoPolicy(policyPath string, data ason.Ason) error {
	policies := make(map[string]string)

	raw, err := os.ReadFile(filepath.Clean(policyPath))
	if err != nil {
		return err
	}

	policies[policyPath] = string(raw)

	compiler, err := regoAst.CompileModulesWithOpt(policies, regoAst.CompileOpts{
		EnablePrintStatements: true,
	})

	var buf bytes.Buffer

	jsonData, err := json.Marshal(data)
	if err != nil {
		return err
	}

	var mapData map[string]interface{}
	if err := json.Unmarshal(jsonData, &mapData); err != nil {
		return err
	}

	r := rego.New(
		rego.Query("data.goray"),
		rego.Compiler(compiler),
		rego.Input(mapData),
		rego.EnablePrintStatements(true),
		rego.PrintHook(topdown.NewPrintHook(&buf)),
	)

	rs, err := r.Eval(context.Background())
	if err != nil {
		return err
	}

	for _, result := range rs {
		for _, r := range result.Expressions {
			fmt.Println(r.Value)
		}
	}

	fmt.Println(buf.String())

	// for _, item := range data {

	// 	var buf bytes.Buffer

	// 	r := rego.New(
	// 		rego.Query("data.goray"),
	// 		rego.Compiler(compiler),
	// 		rego.Input(item),
	// 		rego.EnablePrintStatements(true),
	// 		rego.PrintHook(topdown.NewPrintHook(&buf)),
	// 	)

	// 	rs, err := r.Eval(context.Background())
	// 	if err != nil {
	// 		return err
	// 	}

	// 	for _, result := range rs {
	// 		for _, r := range result.Expressions {
	// 			fmt.Println(r.Value)
	// 		}
	// 	}

	// 	fmt.Println(buf.String())
	// }

	return nil
}

func runOldCmd(cmd *cobra.Command, args []string) {
	// regoClient.ParsePolicyDir("analysis/policy")

	fset := token.NewFileSet()
	f, err := parser.ParseFile(fset, "./ason/testdata/main.go", nil, parser.ParseComments)
	if err != nil {
		log.Fatal(err)
	}

	// startTime := time.Now()
	pass := ason.NewSerPass(fset, ason.SkipComments)
	sf := ason.SerializeFile(pass, f)

	js, _ := json.Marshal(sf)
	fmt.Println(string(js))
	// fmt.Println("Function execution time:", time.Since(startTime))

	// file, err := os.Open(filepath.Clean("./analysis/policy/r1.rego"))
	// if err != nil {
	// 	fmt.Println("Ошибка при открытии файла:", err)
	// 	return
	// }

	fset2 := token.NewFileSet()
	d, err := ason.DeserializeFile(ason.NewDePass(fset2), sf)
	if err != nil {
		log.Fatal(err)
	}

	code, err := GenerateCode(fset2, d)
	if err != nil {
		log.Fatal(err)
	}

	fmt.Println(code)

	// testPolicy, err := openpolicy.NewPolicy(file)

	// policyGroup := []*openpolicy.Policy{testPolicy}

	// policyPath := "./analysis/policy/r1.rego"
	// if err := evaluateRegoPolicy(policyPath, sf); err != nil {
	// 	log.Fatal("Ошибка при выполнении политики:", err)
	// }
}

func GenerateCode(fset *token.FileSet, astFile *ast.File) (string, error) {
	var buf bytes.Buffer

	cfg := printer.Config{
		Mode:     printer.TabIndent | printer.UseSpaces,
		Tabwidth: 2,
	}

	err := cfg.Fprint(&buf, fset, astFile)
	if err != nil {
		return "", err
	}

	return buf.String(), nil
}
