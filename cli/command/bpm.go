package command

import (
	"errors"
	"fmt"
	"log"

	"github.com/4rchr4y/goray/internal/domain/service/tar"
	"github.com/4rchr4y/goray/internal/domain/service/toml"
	svalidator "github.com/4rchr4y/goray/internal/domain/service/validator"
	"github.com/4rchr4y/goray/internal/infra/syswrap"
	"github.com/4rchr4y/goray/internal/ropa/bpm"
	"github.com/4rchr4y/goray/pkg/gvalidate"
	"github.com/go-playground/validator/v10"
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
	Run:   runBuildCmd,
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

func runBuildCmd(cmd *cobra.Command, args []string) {
	sourcePath, destPath := args[0], args[1]

	osWrap := new(syswrap.OsWrapper)
	ioWrap := new(syswrap.IoWrapper)
	fsWrap := new(syswrap.FsWrapper)
	validateClient := svalidator.NewValidatorService(validator.New())
	tomlClient := toml.NewTomlService()
	tarClient := tar.NewTarService(osWrap, fsWrap, ioWrap)
	bpmClient := bpm.NewBpm()
	bpmClient.RegisterCommand(
		bpm.NewValidateCommand(&bpm.ValidateCmdInput{
			Validate: validateClient,
		}),

		bpm.NewBuildCommand(&bpm.BuildCmdInput{
			FsWrap: fsWrap,
			Tar:    tarClient,
			Toml:   tomlClient,
		}),
	)

	file, err := osWrap.Open(fmt.Sprintf("%s/bundle.toml", args[0]))
	if err != nil {
		log.Fatal(err)
		return
	}

	bundlefile, err := bpm.DecodeBundleFile(ioWrap, tomlClient, file)
	if err != nil {
		log.Fatal(err)
		return
	}

	// if err := bundlefile.Validate(validateClient); err != nil {
	// 	log.Fatal(err)
	// 	return
	// }

	buildCommand, err := bpmClient.Command(bpm.BuildCommandName)
	if err != nil {
		log.Fatal(err)
		return
	}

	if err := buildCommand.Execute(&bpm.BuildCmdExecuteInput{
		SourcePath: sourcePath,
		DestPath:   destPath,
		ValidateCmdExecuteInput: &bpm.ValidateCmdExecuteInput{
			BundleFile: bundlefile,
		},
	}); err != nil {
		log.Fatal(err)
		return
	}

	// bpmClient.CreateBundleLockFile(sourcePath)
	// bpmClient.BuildBundle(sourcePath, destPath, "test.bundle")

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
