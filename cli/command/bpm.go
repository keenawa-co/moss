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
	"github.com/4rchr4y/goray/internal/ropa/loader"
	"github.com/4rchr4y/goray/internal/ropa/types"
	"github.com/4rchr4y/goray/pkg/gvalidate"
	"github.com/go-playground/validator/v10"
	"github.com/spf13/cobra"
)

func init() {
	RootCmd.AddCommand(RpmCmd)
	RpmCmd.AddCommand(GetCmd)
	RpmCmd.AddCommand(BuildCmd)

	GetCmd.Flags().BoolP("global", "g", false, "global install")
}

var RpmCmd = &cobra.Command{
	Use:   "bpm",
	Short: "Add a new dependency",
	Long:  ``,
}

// ----------------- Build Command ----------------- //

var BuildCmd = &cobra.Command{
	Use:   "build",
	Short: "Add a new dependency",
	Long:  ``,
	Args:  validateBuildCmdArgs,
	Run:   runBuildCmd,
}

func validateBuildCmdArgs(cmd *cobra.Command, args []string) error {
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
	validateClient := svalidator.NewValidatorService(validator.New())
	tomlClient := toml.NewTomlService()
	tarClient := tar.NewTarService(osWrap, osWrap, ioWrap)
	bpmClient := bpm.NewBpm()

	fsLoaderConf := &loader.FsLoaderConf{
		OsWrap:      osWrap,
		TomlDecoder: tomlClient,
	}

	bpmClient.RegisterCommand(
		bpm.NewValidateCommand(&bpm.ValidateCmdInput{
			Validate: validateClient,
		}),

		bpm.NewBuildCommand(&bpm.BuildCmdInput{
			FsWrap:         osWrap,
			Tar:            tarClient,
			Toml:           tomlClient,
			RegoFileLoader: loader.NewFsLoader(fsLoaderConf),
		}),
	)

	file, err := osWrap.Open(fmt.Sprintf("%s/bundle.toml", args[0]))
	if err != nil {
		log.Fatal(err)
		return
	}

	bundlefile, err := types.DecodeBundleFile(ioWrap, tomlClient, file)
	if err != nil {
		log.Fatal(err)
		return
	}

	buildCommand, err := bpmClient.Command(bpm.BuildCommandName)
	if err != nil {
		log.Fatal(err)
		return
	}

	bundlelock, err := osWrap.Create(fmt.Sprintf("%s/bundle.lock", args[0]))
	if err != nil {
		log.Fatal(err)
		return
	}

	if err := buildCommand.Execute(&bpm.BuildCmdExecuteInput{
		SourcePath: sourcePath,
		DestPath:   destPath,
		BLWriter:   bundlelock,
		ValidateCmdExecuteInput: &bpm.ValidateCmdExecuteInput{
			BundleFile: bundlefile,
		},
	}); err != nil {
		log.Fatal(err)
		return
	}
}

// ----------------- Get Command ----------------- //

var GetCmd = &cobra.Command{
	Use:   "get",
	Short: "Install a new dependency",
	Long:  ``,
	Args:  validateGetCmdArgs,
	Run:   runGetCmd,
}

func validateGetCmdArgs(cmd *cobra.Command, args []string) error {
	if len(args) != 1 {
		return errors.New("wrong number of arguments")
	}

	return nil
}

func runGetCmd(cmd *cobra.Command, args []string) {
	bpmClient := bpm.NewBpm()
	osWrap := new(syswrap.OsWrapper)

	bpmClient.RegisterCommand(
		bpm.NewGetCommand(osWrap),
	)
}
