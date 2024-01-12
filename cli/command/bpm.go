package command

import (
	"errors"
	"fmt"

	"github.com/4rchr4y/goray/internal/domain/service/tar"
	"github.com/4rchr4y/goray/internal/domain/service/toml"
	"github.com/4rchr4y/goray/internal/infra/syswrap"
	"github.com/4rchr4y/goray/internal/ropa/bpm"
	"github.com/4rchr4y/goray/pkg/gvalidate"
	"github.com/spf13/cobra"
)

func init() {
	RootCmd.AddCommand(RpmCmd)
	RpmCmd.AddCommand(InstallCmd)
	RpmCmd.AddCommand(BuildCmd)

	InstallCmd.Flags().BoolP("global", "g", false, "global install")
}

var RpmCmd = &cobra.Command{
	Use:   "rpm",
	Short: "Add a new dependency",
	Long:  ``,
}

var BuildCmd = &cobra.Command{
	Use:   "build",
	Short: "Add a new dependency",
	Long:  ``,
	Args:  validateArgs,
	RunE:  runBuildCmd,
}

func validateArgs(cmd *cobra.Command, args []string) error {
	if len(args) != 2 {
		return errors.New("wrong number of arguments")
	}

	if err := gvalidate.ValidatePath(args[0]); err != nil {
		return fmt.Errorf("invalid path to the package argument: %v", err)
	}

	if err := gvalidate.ValidatePath(args[1]); err != nil {
		return fmt.Errorf("invalid destination path argument: %v", err)
	}

	return nil
}

func runBuildCmd(cmd *cobra.Command, args []string) error {
	sourcePath, destPath := args[0], args[1]

	os := new(syswrap.OsClient)

	ts := toml.NewTomlService()
	tarClient := tar.NewTarService(os, new(syswrap.FsClient), new(syswrap.IoClient))
	bpm := bpm.NewBpm(ts, tarClient, new(syswrap.IoClient))

	file, err := os.Open(fmt.Sprintf("%s/bundle.toml", args[0]))
	if err != nil {
		return err
	}

	bundlefile, err := bpm.ParseBundleFile(file)
	if err != nil {
		return err
	}

	fmt.Println(bundlefile.Package.Name)

	bpm.Pack(sourcePath, destPath, "test.bundle")
	return nil
}

var InstallCmd = &cobra.Command{
	Use:   "install",
	Short: "Install a new dependency",
	Long:  ``,
	RunE:  runAddCmd,
}

func runAddCmd(cmd *cobra.Command, args []string) error {
	// path := args[0]

	return nil
}
