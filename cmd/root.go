package main

import (
	"fmt"

	"github.com/spf13/cobra"
)

var rootCmd = &cobra.Command{
	Use:   "goray",
	Short: "",
	Long:  "",
	Run:   runRootCmd,
}

func init() {
	rootCmd.Flags().BoolP("output", "o", false, "Help message for output")
}

func runRootCmd(cmd *cobra.Command, args []string) {
	fmt.Println("hello, go!")
}
