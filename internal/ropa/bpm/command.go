package bpm

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
	toml        tomlCoder
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

type BuildCmdExecuteInput struct {
	*ValidateCmdExecuteInput

	_          [0]int
	SourcePath string
	DestPath   string
	BLWriter   io.Writer
}

func (cmd *buildCommand) Execute(input interface{}) error {
	typedInput, ok := input.(*BuildCmdExecuteInput)
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
	Tar              tarClient
	Toml             tomlCoder
	RegoFileLoader   regoFileLoader
	BundleLockWriter io.Writer
}

func NewBuildCommand(input *BuildCmdConf) Command {
	bbuilder := &BundleBuilder{
		osWrap: input.OsWrap,
		tar:    input.Tar,
		toml:   input.Toml,
		loader: input.RegoFileLoader,
	}

	return &buildCommand{
		cmdName:     BuildCommandName,
		toml:        input.Toml,
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

type getCmdOsWrapper interface {
	Mkdir(name string, perm fs.FileMode) error
	Stat(name string) (fs.FileInfo, error)
}

type getCommand struct {
	cmdName string
	os      getCmdOsWrapper
}

func (cmd *getCommand) bpmCmd()                  {}
func (cmd *getCommand) Name() string             { return cmd.cmdName }
func (cmd *getCommand) Requires() []string       { return nil }
func (cmd *getCommand) SetCommand(Command) error { return nil }

type GetCmdInput struct {
	HomeDir    string
	BundleFile *types.Bundle
}

func (cmd *getCommand) Execute(input interface{}) error {
	typedInput, ok := input.(*GetCmdInput)
	if !ok {
		return fmt.Errorf("type '%s' is invalid input type for '%s' command", reflect.TypeOf(input), cmd.cmdName)
	}

	bpmDirPath := fmt.Sprintf("%s/%s", typedInput.HomeDir, constant.BPMDir)
	if err := cmd.prepareBpmDir(bpmDirPath); err != nil {
		return fmt.Errorf("error occurred while creating '%s' directory: %v", bpmDirPath, err)
	}

	return nil
}

func (cmd *getCommand) prepareBpmDir(bpmDirPath string) error {
	if _, err := cmd.os.Stat(bpmDirPath); !os.IsNotExist(err) {
		return err
	}

	if err := cmd.os.Mkdir(bpmDirPath, 0755); err != nil {
		return err
	}

	return nil
}

func NewGetCommand(osWrap getCmdOsWrapper) Command {
	return &getCommand{
		cmdName: GetCommandName,
		os:      osWrap,
	}
}
