package command

import (
	"bytes"
	"encoding/json"
	"fmt"
	"go/parser"
	"go/token"
	"os"
	"path/filepath"

	"github.com/4rchr4y/goray/ason"
	"github.com/spf13/cobra"
)

var dumpCmd = &cobra.Command{
	Use:   "dump",
	Short: "Dump AST file in JSON format",
	Long: `Dump AST file in JSON format. 
You need to provide the path and an optional flag for pretty print.`,
	RunE: runDumpCmd,
}

func init() {
	RootCmd.AddCommand(dumpCmd)

	dumpCmd.Flags().StringP("path", "p", "", "Path to the AST file")
	dumpCmd.Flags().BoolP("pretty", "", false, "Pretty print JSON output")

	dumpCmd.MarkFlagRequired("path")
}

func runDumpCmd(cmd *cobra.Command, args []string) error {
	path, err := cmd.Flags().GetString("path")
	if err != nil {
		return fmt.Errorf("error getting 'path' flag: %w", err)
	}

	if filepath.Ext(path) != ".go" {
		return fmt.Errorf("'%s' is not a Go file", path)
	}

	info, err := os.Stat(path)
	if err != nil {
		return fmt.Errorf("error getting file info: %w", err)
	}

	if info.IsDir() {
		return fmt.Errorf("'%s' is a directory, not a Go file", path)
	}

	fset := token.NewFileSet()
	pass := ason.NewSerPass(fset)

	f, err := parser.ParseFile(fset, path, nil, parser.ParseComments)
	if err != nil {
		return fmt.Errorf("error parsing file: %w", err)
	}

	dump := ason.SerializeFile(pass, f)
	jsonData, err := json.Marshal(dump)
	if err != nil {
		return fmt.Errorf("failed marshaling data to JSON: %w", err)
	}

	pretty, err := cmd.Flags().GetBool("pretty")
	if err != nil {
		return fmt.Errorf("error getting 'pretty' flag: %w", err)
	}

	if pretty {
		jsonData, err = prettyprint(jsonData)
		if err != nil {
			return fmt.Errorf("failed to format JSON: %v", err)
		}
	}

	fmt.Println(string(jsonData))
	return nil
}

func prettyprint(b []byte) ([]byte, error) {
	var out bytes.Buffer
	err := json.Indent(&out, b, "", "  ")
	return out.Bytes(), err
}
