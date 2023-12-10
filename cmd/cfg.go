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
	const filepath = "./testdata/test_cfg.toml"
	// conf := config.New(
	// 	config.WithRootDir("./testdata"),
	// 	config.WithRootDir("./testdata"),
	// )

	conf, err := config.NewConfigFromFile(filepath)
	if err != nil {
		log.Fatalf(err.Error())
	}

	fmt.Printf("%#v", conf.Workspace)
}
