package command

import (
	"github.com/spf13/cobra"
)

var addCmd = &cobra.Command{
	Use:   "add",
	Short: "Add a new dependency",
	Long:  ``,
	RunE:  runAddCmd,
}

func init() {
	rpmCmd.AddCommand(addCmd)

	addCmd.Flags().BoolP("global", "g", false, "global install")
}

func runAddCmd(cmd *cobra.Command, args []string) error {
	// path := args[0]

	return nil
}
