package bpm

// --- Temporary documentation ---
// The same principles as in installer.

import (
	"fmt"
	"io"
	"io/fs"
	"os"
	"reflect"
	"strings"

	"github.com/4rchr4y/goray/constant"
	"github.com/4rchr4y/goray/internal/ropa/types"
)

type Command interface {
	Name() string
	Requires() []string
	SetCommand(cmd Command) error
	Execute(input interface{}) error

	bpmCmd()
}

const (
	BuildCommandName    = "build"
	ValidateCommandName = "validate"
	GetCommandName      = "get"
)

// ----------------- Build Command ----------------- //

type tomlCoder interface {
	Decode(data string, v interface{}) error
	Encode(w io.Writer, v interface{}) error
}

type bundleBuilder interface {
	Build(input *BundleBuildInput) error
}

type buildCommand struct {
	cmdName     string
	coder       tomlCoder
	bbuilder    bundleBuilder
	subregistry commandRegistry
}

func (cmd *buildCommand) bpmCmd()      {}
func (cmd *buildCommand) Name() string { return cmd.cmdName }

func (cmd *buildCommand) Requires() []string {
	return []string{
		ValidateCommandName,
	}
}

func (cmd *buildCommand) SetCommand(c Command) error {
	_, ok := cmd.subregistry[c.Name()]
	if ok {
		return fmt.Errorf("command '%s' in '%s' command is already exists", c.Name(), cmd.cmdName)
	}

	cmd.subregistry[c.Name()] = c
	return nil
}

type BuildCmdInput struct {
	*ValidateCmdExecuteInput

	_          [0]int
	SourcePath string
	DestPath   string
	BLWriter   io.Writer
}

func (cmd *buildCommand) Execute(input interface{}) error {
	typedInput, ok := input.(*BuildCmdInput)
	if !ok {
		return fmt.Errorf("type '%s' is invalid input type for '%s' command", reflect.TypeOf(input).Elem().Kind().String(), cmd.cmdName)
	}

	validateCmd := cmd.subregistry[ValidateCommandName]
	if err := validateCmd.Execute(typedInput.ValidateCmdExecuteInput); err != nil {
		return err
	}

	bundleName := strings.ReplaceAll(typedInput.ValidateCmdExecuteInput.BundleFile.Package.Name, ".", "_")
	bbInput := &BundleBuildInput{
		SourcePath: typedInput.SourcePath,
		DestPath:   typedInput.DestPath,
		BundleName: fmt.Sprintf("%s%s", bundleName, constant.BPMBundleExt),
		BLWriter:   typedInput.BLWriter,
	}
	if err := cmd.bbuilder.Build(bbInput); err != nil {
		return err
	}

	return nil
}

type BuildCmdConf struct {
	OsWrap           bbOsWrapper
	Tar              tarCompressor
	TomlCoder        tomlCoder
	RegoFileLoader   regoFileLoader
	BundleLockWriter io.Writer
}

func NewBuildCommand(input *BuildCmdConf) Command {
	bbuilder := &BundleBuilder{
		osWrap:     input.OsWrap,
		compressor: input.Tar,
		coder:      input.TomlCoder,
		loader:     input.RegoFileLoader,
	}

	return &buildCommand{
		cmdName:     BuildCommandName,
		coder:       input.TomlCoder,
		bbuilder:    bbuilder,
		subregistry: make(commandRegistry),
	}
}

// ----------------- Validate Command ----------------- //

type validateClient interface {
	ValidateStruct(s interface{}) error
}

type validateCommand struct {
	cmdName  string
	validate validateClient
}

func (cmd *validateCommand) bpmCmd()                  {}
func (cmd *validateCommand) Name() string             { return cmd.cmdName }
func (cmd *validateCommand) Requires() []string       { return nil }
func (cmd *validateCommand) SetCommand(Command) error { return nil }

type ValidateCmdExecuteInput struct {
	BundleFile *types.BundleFile
}

func (cmd *validateCommand) Execute(input interface{}) error {
	typedInput, ok := input.(*ValidateCmdExecuteInput)
	if !ok {
		return fmt.Errorf("type '%s' is invalid input type for '%s' command", reflect.TypeOf(input), cmd.cmdName)
	}

	if err := typedInput.BundleFile.Validate(cmd.validate); err != nil {
		return fmt.Errorf("failed to execute '%s' command: %v", cmd.cmdName, err)
	}

	return nil
}

func NewValidateCommand(validate validateClient) Command {
	return &validateCommand{
		cmdName:  ValidateCommandName,
		validate: validate,
	}
}

// ----------------- Get Command ----------------- //

type osWrapper interface {
	Mkdir(name string, perm fs.FileMode) error
	Stat(name string) (fs.FileInfo, error)
	MkdirAll(path string, perm fs.FileMode) error
	Create(name string) (*os.File, error)
	UserHomeDir() (string, error)
	WriteFile(name string, data []byte, perm fs.FileMode) error
}

type tomlEncoder interface {
	Encode(w io.Writer, v interface{}) error
}

type bundleInstaller interface {
	Install(input *BundleInstallInput) error
}

type fsLoader interface {
	LoadBundle(path string) (*types.Bundle, error)
}

type getCommand struct {
	cmdName   string
	encoder   tomlEncoder
	installer bundleInstaller
	loader    fsLoader
	osWrap    osWrapper
}

func (cmd *getCommand) bpmCmd()                  {}
func (cmd *getCommand) Name() string             { return cmd.cmdName }
func (cmd *getCommand) Requires() []string       { return nil }
func (cmd *getCommand) SetCommand(Command) error { return nil }

type GetCmdInput struct {
	BundlePath string
}

func (cmd *getCommand) Execute(input interface{}) error {
	typedInput, ok := input.(*GetCmdInput)
	if !ok {
		return fmt.Errorf("type '%s' is invalid input type for '%s' command", reflect.TypeOf(input), cmd.cmdName)
	}

	homeDir, err := cmd.osWrap.UserHomeDir()
	if err != nil {
		return err
	}

	bundle, err := cmd.loader.LoadBundle(typedInput.BundlePath)
	if err != nil {
		return err
	}

	bpmDirPath := fmt.Sprintf("%s/%s", homeDir, constant.BPMDir)
	bundleName := bundle.BundleFile.Package.Name
	bundleVersion := bundle.BundleFile.Package.Version
	bundleVersionDir := fmt.Sprintf("%s/%s/%s", bpmDirPath, bundleName, bundleVersion)

	if !cmd.isAlreadyInstalled(bundleVersionDir) {
		fmt.Printf("Bundle '%s' with version '%s' is already installed\n", bundleName, bundleVersion)
		return nil
	}

	// creating all the directories that are necessary to save files
	if err := cmd.osWrap.MkdirAll(bundleVersionDir, 0755); err != nil {
		return err
	}

	if err := cmd.installer.Install(&BundleInstallInput{
		Dir:    bundleVersionDir,
		Bundle: bundle,
	}); err != nil {
		return fmt.Errorf("can't install bundle '%s': %v", bundleName, err)
	}

	return nil
}

func (cmd *getCommand) isAlreadyInstalled(bundleVersionDir string) bool {
	_, err := cmd.osWrap.Stat(bundleVersionDir)
	return os.IsNotExist(err)
}

type GetCmdConf struct {
	OsWrap      osWrapper
	TomlEncoder tomlEncoder
	FileLoader  fsLoader
}

func NewGetCommand(conf *GetCmdConf) Command {
	return &getCommand{
		cmdName: GetCommandName,
		osWrap:  conf.OsWrap,
		encoder: conf.TomlEncoder,
		loader:  conf.FileLoader,
		installer: &BundleInstaller{
			osWrap:  conf.OsWrap,
			encoder: conf.TomlEncoder,
		},
	}
}
