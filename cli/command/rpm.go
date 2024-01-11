package command

import "github.com/spf13/cobra"

var RpmCmd = &cobra.Command{
	Use:   "add",
	Short: "Add a new dependency",
	Long:  ``,
	// RunE:  runAddCmd,
}
