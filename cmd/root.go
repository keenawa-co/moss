package main

import (
	"bytes"
	"context"
	"fmt"
	"go/ast"
	"go/parser"
	"go/printer"
	"go/token"
	"log"
	"os"
	"path/filepath"
	"reflect"

	regoAst "github.com/open-policy-agent/opa/ast"
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

// Функция для обхода AST и сбора данных
func inspectFile(fset *token.FileSet, f *ast.File) []map[string]interface{} {
	var data []map[string]interface{}

	ast.Inspect(f, func(n ast.Node) bool {
		// Ищем выражения
		if exprStmt, ok := n.(*ast.ExprStmt); ok {
			if callExpr, ok := exprStmt.X.(*ast.CallExpr); ok {
				if selExpr, ok := callExpr.Fun.(*ast.SelectorExpr); ok {
					if ident, ok := selExpr.X.(*ast.Ident); ok {
						if ident.Name == "fmt" && selExpr.Sel.Name == "Println" || selExpr.Sel.Name == "Sprintln" {

							data = append(data, map[string]interface{}{
								"Kind": "ExprStmt",
								"Node": map[string]interface{}{
									"X": map[string]interface{}{
										"Fun": map[string]interface{}{
											"X": map[string]interface{}{
												"Name":    ident.Name,
												"NamePos": fset.Position(ident.NamePos).Line,
											},
											"Sel": map[string]interface{}{
												"Name": selExpr.Sel.Name,
											},
										},
									},
								},
							})
						}
					}
				}
			}
		}
		return true
	})

	return data
}

func evaluateRegoPolicy(policyPath string, data []map[string]interface{}) error {
	policies := make(map[string]string)

	raw, err := os.ReadFile(filepath.Clean(policyPath))
	if err != nil {
		return err
	}

	policies[policyPath] = string(raw)

	compiler, err := regoAst.CompileModulesWithOpt(policies, regoAst.CompileOpts{
		EnablePrintStatements: true,
	})

	as := compiler.GetAnnotationSet()
	fmt.Println(len(as.Flatten()))
	for _, entry := range as.Flatten() {
		fmt.Printf("%#v", entry)
		fmt.Printf("%v at %v has annotations %v\n",
			entry.Path,
			entry.Location,
			entry.Annotations)
	}

	for _, item := range data {

		var buf bytes.Buffer

		r := rego.New(
			rego.Query("data.goray"),
			rego.Compiler(compiler),
			rego.Input(item),
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
	}

	return nil
}

func StructToMap(v interface{}) map[string]interface{} {
	result := make(map[string]interface{})

	val := reflect.ValueOf(v)
	if val.Kind() == reflect.Ptr {
		val = val.Elem()
	}

	typ := val.Type()
	for i := 0; i < val.NumField(); i++ {
		field := typ.Field(i)
		value := val.Field(i)
		result[field.Name] = value.Interface()
	}

	return result
}

func runRootCmd(cmd *cobra.Command, args []string) {
	// regoClient.ParsePolicyDir("analysis/policy")

	fset := token.NewFileSet()

	f, err := parser.ParseFile(fset, "./ason/testdata/main.go", nil, parser.ParseComments)
	if err != nil {
		log.Fatal(err)
	}

	// startTime := time.Now()
	// pass := ason.NewSerPass(fset)
	// sf := ason.SerializeFile(pass, f)
	// fmt.Println("Function execution time:", time.Since(startTime))

	// js, err := json.Marshal(sf)
	// if err != nil {
	// 	log.Fatal(err)
	// }

	// fmt.Println(string(js))

	// fmt.Println()
	// fmt.Println()

	// fset2 := token.NewFileSet()
	// df, _ := ason.DeserializeFile(ason.NewDePass(fset2), sf)

	// code, err := GenerateCode(fset2, df)
	// if err != nil {
	// 	log.Fatal(err)
	// }

	// fmt.Println(code)

	data := inspectFile(fset, f)

	policyPath := "./analysis/policy/r1.rego"
	if err := evaluateRegoPolicy(policyPath, data); err != nil {
		log.Fatal("Ошибка при выполнении политики:", err)
	}
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
