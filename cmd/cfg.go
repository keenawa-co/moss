package main

import (
	"fmt"
	"log"

	"github.com/4rchr4y/goray/config"
	"github.com/spf13/cobra"
)

var cfgCmd = &cobra.Command{
	Use:   "cfg",
	Short: "",
	Long:  "",
	Run:   runCfgCmd,
}

func init() {
	rootCmd.AddCommand(cfgCmd)
}

func runCfgCmd(cmd *cobra.Command, args []string) {
	const filepath string = "./testdata/test_cfg.toml"
	cfg, err := config.NewFromFile(filepath)
	if err != nil {
		log.Fatalf(err.Error())
	}

	fmt.Printf("%v", cfg.Analysis)
}
