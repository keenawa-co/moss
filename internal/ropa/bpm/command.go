package bpm

import (
	"fmt"
	"io"
	"os"
	"path/filepath"
	"reflect"
	"strings"
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
)

// ----------------- Build Command ----------------- //

type fsWrapper interface {
	Walk(root string, fn filepath.WalkFunc) error
}

type tarClient interface {
	Compress(dirPath string, targetDir string, archiveName string) error
}

type tomlClient interface {
	Decode(data string, v interface{}) error
	Encode(w io.Writer, v interface{}) error
}

type buildCommand struct {
	cmdName     string
	fswrap      fsWrapper
	tar         tarClient
	toml        tomlClient
	subregistry cmdRegistry
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

	SourcePath string
	DestPath   string
}

func (cmd *buildCommand) Execute(input interface{}) error {
	typedInput, ok := input.(*BuildCmdExecuteInput)
	if !ok {
		return fmt.Errorf("type '%s' is invalid input type for '%s' command", reflect.TypeOf(input).Elem().Kind().String(), cmd.cmdName)
	}

	if err := cmd.subregistry[ValidateCommandName].Execute(typedInput.ValidateCmdExecuteInput); err != nil {
		return err
	}

	if err := cmd.buildBundle(typedInput.SourcePath, typedInput.DestPath, "test.bundle"); err != nil {
		return err
	}

	return nil
}

type BuildCmdInput struct {
	FsWrap fsWrapper
	Tar    tarClient
	Toml   tomlClient
}

func NewBuildCommand(input *BuildCmdInput) Command {
	return &buildCommand{
		cmdName:     BuildCommandName,
		fswrap:      input.FsWrap,
		tar:         input.Tar,
		toml:        input.Toml,
		subregistry: make(cmdRegistry),
	}
}

func (cmd *buildCommand) createBundleLockFile(dirPath string) (*BundleLock, error) {
	walkFunc := func(path string, info os.FileInfo, err error) error {
		if err != nil {
			fmt.Printf("prevent panic by handling failure accessing a path %q: %v\n", path, err)
			return err
		}

		if !info.IsDir() && strings.HasSuffix(info.Name(), ".rego") {
			fmt.Println(path)
		}
		return nil
	}

	err := cmd.fswrap.Walk(dirPath, walkFunc)
	if err != nil {
		fmt.Printf("error walking the path %q: %v\n", dirPath, err)
	}

	return nil, nil
}

func (cmd *buildCommand) buildBundle(sourcePath string, destPath string, bundleName string) error {
	if err := cmd.tar.Compress(sourcePath, destPath, bundleName); err != nil {
		return fmt.Errorf("error occurred while building '%s' bundle: %v", bundleName, err)
	}

	return nil
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
	BundleFile *BundleFile
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

type ValidateCmdInput struct {
	Validate validateClient
}

func NewValidateCommand(input *ValidateCmdInput) Command {
	return &validateCommand{
		cmdName:  ValidateCommandName,
		validate: input.Validate,
	}
}
