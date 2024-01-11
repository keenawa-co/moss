package command

import "github.com/spf13/cobra"

var rpmCmd = &cobra.Command{
	Use:   "add",
	Short: "Add a new dependency",
	Long:  ``,
	// RunE:  runAddCmd,
}
