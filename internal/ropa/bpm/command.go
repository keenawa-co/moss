package bpm

import (
	"fmt"
	"io"
	"io/fs"
	"os"
	"path/filepath"
	"reflect"
	"strings"

	"github.com/4rchr4y/goray/constant"
	"github.com/4rchr4y/goray/internal/ropa/loader"
	"github.com/4rchr4y/goray/internal/ropa/types"
	"github.com/4rchr4y/goray/version"
)

type Command interface {
	Name() string
	Requires() []string
	SetCommand(cmd Command) error
	Execute(input interface{}) (interface{}, error)

	bpmCmd()
}

const (
	BuildCommandName    = "build"
	ValidateCommandName = "validate"
	GetCommandName      = "get"
)

// ----------------- Build Command ----------------- //

type buildCmdTarCompressor interface {
	Compress(dirPath string, targetDir string, archiveName string) error
}

type buildCommand struct {
	cmdName     string
	compressor  buildCmdTarCompressor
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
	_          [0]int
	SourcePath string
	DestPath   string
	BLWriter   io.Writer
}

func (cmd *buildCommand) Execute(input interface{}) (interface{}, error) {
	typedInput, ok := input.(*BuildCmdInput)
	if !ok {
		return nil, fmt.Errorf("type '%s' is invalid input type for '%s' command", reflect.TypeOf(input).Elem().Kind().String(), cmd.cmdName)
	}

	validateCmd := cmd.subregistry[ValidateCommandName]
	validateCmdInput := &ValidateCmdInput{
		SourcePath: typedInput.SourcePath,
	}

	rawResult, err := validateCmd.Execute(validateCmdInput)
	if err != nil {
		return nil, err
	}

	result, ok := rawResult.(*ValidateCmdResult)
	if !ok {
		return nil, fmt.Errorf("type '%s' is invalid type for '%s' command result", reflect.TypeOf(input).Elem().Kind().String(), ValidateCommandName)
	}

	if err := cmd.collectVendors(result.Bundle.BundleFile); err != nil {
		return nil, err
	}

	fmt.Println(filepath.Abs(typedInput.SourcePath))

	bundleFileName := fmt.Sprintf("%s%s", result.Bundle.BundleFile.Package.Name, constant.BPMBundleExt)
	if err := cmd.compressor.Compress(typedInput.SourcePath, typedInput.DestPath, bundleFileName); err != nil {
		return nil, fmt.Errorf("error occurred while building '%s' bundle: %v", bundleFileName, err)
	}

	return nil, nil
}

func (cmd *buildCommand) collectVendors(bundlefile *types.BundleFile) error {
	for name, d := range bundlefile.Dependencies {
		fmt.Println(name, d.Source)

		absolutePath, err := filepath.Abs(d.Source)
		if err != nil {
			return err
		}

		fmt.Println(absolutePath)

	}

	return nil
}

type BuildCmdConf struct {
	TarCompressor buildCmdTarCompressor
}

func NewBuildCommand(conf *BuildCmdConf) Command {
	return &buildCommand{
		cmdName:     BuildCommandName,
		compressor:  conf.TarCompressor,
		subregistry: make(commandRegistry),
	}
}

// ----------------- Validate Command ----------------- //

type validateCmdValidator interface {
	ValidateStruct(s interface{}) error
}

type validateCmdOSWrapper interface {
	Create(name string) (*os.File, error)
	Walk(root string, fn filepath.WalkFunc) error
	Open(name string) (*os.File, error)
}

type validateCmdIOWrapper interface {
	ReadAll(reader io.Reader) ([]byte, error)
}

type validateCmdTomler interface {
	Encode(writer io.Writer, value interface{}) error
	Decode(data string, value interface{}) error
}

type validateCmdBundleParser interface {
	Parse(input *loader.ParseInput) (*types.Bundle, error)
}

type validateCommand struct {
	cmdName  string
	osWrap   validateCmdOSWrapper
	ioWrap   validateCmdIOWrapper
	tomler   validateCmdTomler
	bparser  validateCmdBundleParser
	validate validateCmdValidator
}

func (cmd *validateCommand) bpmCmd()                  {}
func (cmd *validateCommand) Name() string             { return cmd.cmdName }
func (cmd *validateCommand) Requires() []string       { return nil }
func (cmd *validateCommand) SetCommand(Command) error { return nil }

type ValidateCmdInput struct {
	SourcePath string
}

type ValidateCmdResult struct {
	Bundle *types.Bundle
}

func (cmd *validateCommand) Execute(input interface{}) (interface{}, error) {
	typedInput, ok := input.(*ValidateCmdInput)
	if !ok {
		return nil, fmt.Errorf("type '%s' is invalid input type for '%s' command", reflect.TypeOf(input), cmd.cmdName)
	}

	files, err := cmd.loadBundleDir(typedInput.SourcePath)
	if err != nil {
		return nil, err
	}

	parserInput := &loader.ParseInput{
		FileName: typedInput.SourcePath,
		Files:    files,
	}

	bundle, err := cmd.bparser.Parse(parserInput)
	if err != nil {
		return nil, err
	}

	updated := bundle.UpdateLock()
	if !updated {
		return &ValidateCmdResult{
			Bundle: bundle,
		}, nil
	}

	bundleLockFile, err := cmd.osWrap.Create(fmt.Sprintf("%s/%s", typedInput.SourcePath, constant.BPMLockFile))
	if err != nil {
		return nil, err
	}

	if err := cmd.tomler.Encode(bundleLockFile, bundle.BundleLockFile); err != nil {
		return nil, err
	}

	return &ValidateCmdResult{
		Bundle: bundle,
	}, nil
}

func (cmd *validateCommand) createBundleLockFile(files map[string]*types.RawRegoFile) (*types.BundleLockFile, error) {
	bundlelock := &types.BundleLockFile{
		Version: version.BPM,
		Modules: make([]*types.ModuleDef, len(files)),
	}

	var i uint
	for path, file := range files {
		bundlelock.Modules[i] = &types.ModuleDef{
			Name:     file.Parsed.Package.Path.String(),
			Source:   path,
			Checksum: file.Sum(),
			Dependencies: func() []string {
				result := make([]string, len(file.Parsed.Imports))
				for i, _import := range file.Parsed.Imports {
					result[i] = _import.Path.String()
				}

				return result
			}(),
		}

		i++
	}

	return bundlelock, nil
}

func (cmd *validateCommand) loadBundleDir(dirPath string) (map[string][]byte, error) {
	files := make(map[string][]byte)

	walkFunc := func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return fmt.Errorf("error occurred while accessing a path %s: %v", path, err)
		}

		if !info.IsDir() {
			file, err := cmd.osWrap.Open(path)
			if err != nil {
				return err
			}

			content, err := cmd.ioWrap.ReadAll(file)
			if err != nil {
				return err
			}

			localPath := strings.Clone(path[len(dirPath)+1:])
			files[localPath] = content
		}

		return nil
	}

	err := cmd.osWrap.Walk(dirPath, walkFunc)
	if err != nil {
		return nil, fmt.Errorf("error walking the path %s: %v", dirPath, err)
	}

	return files, nil
}

type ValidateCmdConf struct {
	OsWrap       validateCmdOSWrapper
	IoWrap       validateCmdIOWrapper
	Tomler       validateCmdTomler
	BundleParser validateCmdBundleParser
	Validator    validateCmdValidator
}

func NewValidateCommand(conf *ValidateCmdConf) Command {
	return &validateCommand{
		cmdName:  ValidateCommandName,
		osWrap:   conf.OsWrap,
		ioWrap:   conf.IoWrap,
		tomler:   conf.Tomler,
		bparser:  conf.BundleParser,
		validate: conf.Validator,
	}
}

// ----------------- Get Command ----------------- //

type getCmdOSWrapper interface {
	Mkdir(name string, perm fs.FileMode) error
	Stat(name string) (fs.FileInfo, error)
	MkdirAll(path string, perm fs.FileMode) error
	Create(name string) (*os.File, error)
	UserHomeDir() (string, error)
	WriteFile(name string, data []byte, perm fs.FileMode) error
}

type getCmdTomler interface {
	Encode(w io.Writer, v interface{}) error
}

type getCmdBundleInstaller interface {
	Install(input *BundleInstallInput) error
}

type getCmdLoader interface {
	// DownloadBundle(url string, tag string) (*loader.DownloadResult, error)
}

type getCommand struct {
	cmdName   string
	encoder   getCmdTomler
	installer getCmdBundleInstaller
	loader    getCmdLoader
	osWrap    getCmdOSWrapper
}

func (cmd *getCommand) bpmCmd()                  {}
func (cmd *getCommand) Name() string             { return cmd.cmdName }
func (cmd *getCommand) Requires() []string       { return nil }
func (cmd *getCommand) SetCommand(Command) error { return nil }

type GetCmdInput struct {
	Version string // bundle package version
	URL     string // url to download bundle
}

func (cmd *getCommand) Execute(rawInput interface{}) (interface{}, error) {
	// input, ok := rawInput.(*GetCmdInput)
	// if !ok {
	// 	return nil, fmt.Errorf("type '%s' is invalid input type for '%s' command", reflect.TypeOf(rawInput), cmd.cmdName)
	// }

	// bundle, err := cmd.loader.DownloadBundle(input.URL, input.Version)
	// if err != nil {
	// 	return nil, err
	// }

	// fmt.Println(bundle.Bundle.BundleFile.Package.Name)

	// homeDir, err := cmd.osWrap.UserHomeDir()
	// if err != nil {
	// 	return nil, err
	// }

	// bundle, err := cmd.loader.LoadBundle(typedInput.BundlePath)
	// if err != nil {
	// 	return nil, err
	// }

	// bpmDirPath := fmt.Sprintf("%s/%s", homeDir, constant.BPMDir)
	// bundleName := bundle.BundleFile.Package.Name
	// bundleVersion := bundle.BundleFile.Package.Version
	// bundleVersionDir := fmt.Sprintf("%s/%s/%s", bpmDirPath, bundleName, bundleVersion)

	// if !cmd.isAlreadyInstalled(bundleVersionDir) {
	// 	fmt.Printf("Bundle '%s' with version '%s' is already installed\n", bundleName, bundleVersion)
	// 	return nil, nil
	// }

	// // creating all the directories that are necessary to save files
	// if err := cmd.osWrap.MkdirAll(bundleVersionDir, 0755); err != nil {
	// 	return nil, err
	// }

	// if err := cmd.installer.Install(&BundleInstallInput{
	// 	Dir:    bundleVersionDir,
	// 	Bundle: bundle,
	// }); err != nil {
	// 	return nil, fmt.Errorf("can't install bundle '%s': %v", bundleName, err)
	// }

	return nil, nil
}

func (cmd *getCommand) isAlreadyInstalled(bundleVersionDir string) bool {
	_, err := cmd.osWrap.Stat(bundleVersionDir)
	return os.IsNotExist(err)
}

type GetCmdConf struct {
	OsWrap      getCmdOSWrapper
	TomlEncoder getCmdTomler
	FileLoader  getCmdLoader
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
