package rpm

import (
	"github.com/4rchr4y/goray/cmd/root"
	"github.com/spf13/cobra"
)

var AddCmd = &cobra.Command{
	Use:   "add",
	Short: "Add a new dependency",
	Long:  ``,
	RunE:  runAddCmd,
}

func init() {
	root.RootCmd.AddCommand(AddCmd)

	AddCmd.Flags().BoolP("global", "g", false, "global install")
}

func runAddCmd(cmd *cobra.Command, args []string) error {
	// path := args[0]

	return nil
}
