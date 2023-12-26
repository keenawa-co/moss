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
	"github.com/4rchr4y/goray/internal/syswrap"
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

var policies = []*config.PolicyDef{
	{
		Path:    "opa/policy/r1.rego",
		Target:  []string{"testdata/main.go"},
		Include: []string{"testdata/test.rego"},
	},

	// {
	// 	Path:    "opa/policy/r2.rego",
	// 	Target:  []string{"testdata/main.go"},
	// 	Include: []string{"testdata/test.rego"},
	// },
}

func runRootCmd(cmd *cobra.Command, args []string) {
	loader := openpolicy.NewLoader(new(syswrap.FsClient))
	machine := openpolicy.NewMachine(loader)

	b, err := loader.LoadBundle("bundle.tar.gz")
	if err != nil {
		log.Fatal(err)
		return
	}

	machine.RegisterBundle(b)

	for i, v := range policies {
		file, err := loader.LoadRegoFile(v.Path)
		if err != nil {
			log.Fatal(err)
			return
		}

		if err := machine.RegisterPolicy(&openpolicy.Policy{
			File:    file,
			Targets: policies[i].Target,
			Vendors: policies[i].Include,
		}); err != nil {
			log.Fatal(err)
			return
		}
	}

	compilers, err := machine.Compile()

	for _, v := range compilers[0].Modules {
		fmt.Println(v.Package)
	}

	fileMap, err := tmpGetFileAstAsMap("testdata/main.go")
	if err != nil {
		log.Fatal(err)
		return
	}

	var buf bytes.Buffer
	r := rego.New(
		rego.Query("data.goray"),
		rego.Compiler(compilers[0]),
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
