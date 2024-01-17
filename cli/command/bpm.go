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
	"github.com/4rchr4y/goray/pkg/gvalidate"
	"github.com/go-playground/validator/v10"
	"github.com/spf13/cobra"
)

func init() {
	RootCmd.AddCommand(BpmCmd)
	BpmCmd.AddCommand(GetCmd)
	BpmCmd.AddCommand(BuildCmd)

	GetCmd.Flags().BoolP("global", "g", false, "global install")
}

var BpmCmd = &cobra.Command{
	Use:   "bpm",
	Short: "Bundle Package Manager",
	Long:  ``,
}

// ----------------- Build Command ----------------- //

var BuildCmd = &cobra.Command{
	Use:   "build",
	Short: "Build a new bundle",
	Long:  ``,
	Args:  validateBuildCmdArgs,
	Run:   runBuildCmd,
}

func validateBuildCmdArgs(cmd *cobra.Command, args []string) error {
	if len(args) != 2 {
		return fmt.Errorf("wrong number of arguments, expect %d got %d", 2, len(args))
	}

	if err := gvalidate.ValidatePath(args[0]); err != nil {
		return fmt.Errorf("'%s' is invalid path to the source folder: %v", args[1], err)
	}

	if err := gvalidate.ValidatePath(args[1]); err != nil {
		return fmt.Errorf("'%s' is invalid path to the destination folder: %v", args[1], err)
	}

	return nil
}

func runBuildCmd(cmd *cobra.Command, args []string) {
	sourcePath, destPath := args[0], args[1]

	osWrap := new(syswrap.OsWrapper)
	ioWrap := new(syswrap.IoWrapper)
	validateClient := svalidator.NewValidatorService(validator.New())
	tomler := toml.NewTomlService()
	tarClient := tar.NewTarService(osWrap, osWrap, ioWrap)
	bpmClient := bpm.NewBpm()

	bpmClient.RegisterCommand(
		bpm.NewValidateCommand(&bpm.ValidateCmdConf{
			OsWrap:       osWrap,
			IoWrap:       ioWrap,
			Tomler:       tomler,
			BundleParser: loader.NewBundleParser(tomler),
			Validator:    validateClient,
		}),
		bpm.NewBuildCommand(&bpm.BuildCmdConf{
			// OsWrap:        osWrap,
			TarCompressor: tarClient,

			// TomlCoder:     tomlClient,
			// RegoFileLoader: loader.NewFsLoader(&loader.FsLoaderConf{
			// 	OsWrap:      osWrap,
			// 	TomlDecoder: tomlClient,
			// }),
		}),
	)

	// file, err := osWrap.Open(fmt.Sprintf("%s/%s", args[0], constant.BPMFile))
	// if err != nil {
	// 	log.Fatal(err)
	// 	return
	// }

	// content, err := ioWrap.ReadAll(file)
	// if err != nil {
	// 	log.Fatal(err)
	// 	return
	// }

	// bundlefile := new(types.BundleFile)
	// if err := tomlClient.Decode(string(content), bundlefile); err != nil {
	// 	log.Fatal(err)
	// 	return
	// }

	buildCommand, err := bpmClient.Command(bpm.BuildCommandName)
	if err != nil {
		log.Fatal(err)
		return
	}

	if _, err := buildCommand.Execute(&bpm.BuildCmdInput{
		SourcePath: sourcePath,
		DestPath:   destPath,
		// ValidateCmdExecuteInput: &bpm.ValidateCmdExecuteInput{
		// 	BundleFile: bundlefile,
		// },
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

	if err := gvalidate.ValidatePath(args[0]); err != nil {
		return fmt.Errorf("'%s' is invalid path to the bundle file: %v", args[0], err)
	}

	return nil
}

func runGetCmd(cmd *cobra.Command, args []string) {
	pathToBundle := args[0]
	bpmClient := bpm.NewBpm()
	osWrap := new(syswrap.OsWrapper)
	ioWrap := new(syswrap.IoWrapper)
	tomlCoder := toml.NewTomlService()

	fsLoader := loader.NewFsLoader(&loader.FsLoaderConf{
		OsWrap:      osWrap,
		IoWrap:      ioWrap,
		TomlDecoder: toml.NewTomlService(),
	})

	bpmClient.RegisterCommand(
		bpm.NewGetCommand(&bpm.GetCmdConf{
			OsWrap:      osWrap,
			TomlEncoder: tomlCoder,
			FileLoader:  fsLoader,
		}),
	)

	getCmd, err := bpmClient.Command(bpm.GetCommandName)
	if err != nil {
		log.Fatal(err)
		return
	}

	if _, err := getCmd.Execute(&bpm.GetCmdInput{
		BundlePath: pathToBundle,
	}); err != nil {
		log.Fatal(err)
		return
	}
}
