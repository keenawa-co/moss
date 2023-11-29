package main

import (
	"log"

	"github.com/4rchr4y/goray/plugin"
	"github.com/spf13/cobra"
	"golang.org/x/tools/go/analysis/multichecker"
)

var rootCmd = &cobra.Command{
	Use:   "goray",
	Short: "",
	Long:  "",
	Run:   runRootCmd,
}

func init() {
	a1, err := plugin.LoadPlugin("./analysis/a001/a001.so")
	if err != nil {
		log.Fatal(err)
	}

	// a2, err := plugin.LoadPlugin("./analysis/a002/a002.so")
	// if err != nil {
	// 	log.Fatal(err)
	// }

	multichecker.Main(
		a1,
	)

	rootCmd.Flags().BoolP("output", "o", false, "Help message for output")
}

func runRootCmd(cmd *cobra.Command, args []string) {

	// cfg := &packages.Config{
	// 	Mode: packages.LoadAllSyntax,
	// 	// Dir:  "./...",
	// }

	// pkgs, err := packages.Load(cfg, "./...")
	// if err != nil {
	// 	log.Fatalf("Error: %v", err)
	// }

	// for _, pkg := range pkgs {
	// 	fmt.Println(pkg.String())
	// 	// pass := &analysis.Pass{
	// 	// 	Analyzer:   a1,
	// 	// 	Pkg:        pkg.Types,
	// 	// 	TypesInfo:  pkg.TypesInfo,
	// 	// 	Fset:       pkg.Fset,
	// 	// 	Files:      pkg.Syntax,
	// 	// 	Report:     func(diag analysis.Diagnostic) { log.Print(diag) },
	// 	// 	ResultOf:   make(map[*analysis.Analyzer]interface{}),
	// 	// 	OtherFiles: pkg.OtherFiles,
	// 	// }

	// 	fmt.Println("---", len(a1.Requires))
	// 	// _, err := a1.Run(pass)
	// 	// if err != nil {
	// 	// 	log.Printf(pkg.Name, err)
	// 	// }

	// 	// fmt.Println(r.URL)

	// }

}
