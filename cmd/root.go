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
	"reflect"

	"github.com/4rchr4y/goray/ason"
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

func init() {
	// a1, err := plugin.LoadPlugin("./analysis/a001/a001.so")
	// if err != nil {
	// 	log.Fatal(err)
	// }

	// a2, err := plugin.LoadPlugin("./analysis/a002/a002.so")
	// if err != nil {
	// 	log.Fatal(err)
	// }

	// multichecker.Main(
	// 	a1,
	// )

	// rootCmd.Flags().BoolP("output", "o", false, "Help message for output")
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
	for _, item := range data {

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
		// fmt.Println(rs)
		// if len(rs) == 0 || len(rs[0].Expressions) == 0 {
		// 	log.Printf("no result")
		// 	return nil
		// }

		// var out evalOutput

		// raw, err := json.Marshal(rs[0].Expressions[0].Value)
		// if err != nil {
		// 	return err

		// }
		// if err := json.Unmarshal(raw, &out); err != nil {
		// 	return err
		// }

		// fmt.Println(len(out.Fail))

		// for _, fail := range out.Fail {
		// 	fmt.Println(fail.Msg)

		// }
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

	// mr := ason.NewMarshaller(ason.Options{
	// 	WithImports:   true,
	// 	WithPositions: true,
	// 	WithComments:  true,
	// })

	fset := token.NewFileSet()

	f, err := parser.ParseFile(fset, "./testdata/main.go", nil, parser.AllErrors)
	if err != nil {
		log.Fatal(err)
	}

	sf := ason.SerializeFile(ason.NewSerPass(fset), f)

	js, err := json.Marshal(sf)
	if err != nil {
		log.Fatal(err)
	}

	fmt.Println(string(js))

	fmt.Println()
	fmt.Println()

	fset2 := token.NewFileSet()
	df := ason.DeserializeFile(ason.NewDePass(fset2), sf)

	code, err := GenerateCode(fset2, df)
	if err != nil {
		log.Fatal(err)
	}

	fmt.Println(code)

	// ast.Print(token.NewFileSet(), ds)

	// ast.Inspect(f, func(n ast.Node) bool {
	// 	if n == nil {
	// 		return true
	// 	}
	// 	if ident, ok := n.(*ast.Ident); ok && ident.Obj != nil {

	// 		js, err := json.Marshal(o)
	// 		if err != nil {
	// 			log.Fatal(err)
	// 		}

	// 		fmt.Println(string(js))
	// 	}
	// 	return true
	// })

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
