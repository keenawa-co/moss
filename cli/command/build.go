package command

import "github.com/spf13/cobra"

var buildCmd = &cobra.Command{
	Use:   "add",
	Short: "Add a new dependency",
	Long:  ``,
	RunE:  runBuildCmd,
}

func init() {
	rpmCmd.AddCommand(addCmd)

}

func runBuildCmd(cmd *cobra.Command, args []string) error {

	return nil
}
