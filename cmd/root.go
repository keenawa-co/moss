package cmd

import (
	"fmt"
	"os"

	"github.com/spf13/cobra"
)

var rootCmd = &cobra.Command{
	Use:   "goray",
	Short: "",
	Long:  "",
	Run:   run,
}

func Execute() {
	err := rootCmd.Execute()
	if err != nil {
		os.Exit(1)
	}
}

func init() {
	rootCmd.Flags().BoolP("output", "o", false, "Help message for output")
}

func run(cmd *cobra.Command, args []string) {
	fmt.Println("hello, go!")
}
