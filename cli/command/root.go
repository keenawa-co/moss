package command

import (
	"encoding/json"
	"fmt"
	"go/parser"
	"go/token"
	"log"

	"github.com/4rchr4y/goray/analysis/openpolicy"
	"github.com/4rchr4y/goray/ason"
	"github.com/4rchr4y/goray/internal/infra/db/badger"
	"github.com/4rchr4y/goray/internal/infra/syswrap"
	"github.com/4rchr4y/goray/internal/ropa"
	"github.com/4rchr4y/goray/internal/ropa/loader"
	"github.com/4rchr4y/goray/pkg/radix"
	"github.com/4rchr4y/goray/rayfile"
	"github.com/spf13/cobra"
)

var RootCmd = &cobra.Command{
	Use:   "goray",
	Short: "",
	Long:  "",
	Run:   runRootCmd,
}

type failCase struct {
	Msg string `json:"msg"`
	Pos int    `json:"pos"`
	Sev string `json:"sev"`
}

type evalOutput struct {
	Fail []*failCase `json:"fail"`
}

var policies = []*rayfile.PolicyDef{
	{
		Path:         "builtin/opa/policy/r1.rego",
		Target:       []string{"testdata/main.go"},
		Dependencies: []string{"testdata/test.rego"},
		// Dependencies: []string{"testdata"},
	},

	// {
	// 	Path:    "opa/policy/r2.rego",
	// 	Target:  []string{"testdata/main.go"},
	// 	Include: []string{"testdata/test.rego"},
	// },
}

func mapIncludeToVendorDesc(includes []string) []*openpolicy.VendorDescription {
	result := make([]*openpolicy.VendorDescription, len(includes))

	for i := range includes {
		result[i] = &openpolicy.VendorDescription{
			Path: includes[i],
			Type: openpolicy.TypeRegoFile,
		}
	}

	return result
}

func runRootCmd(cmd *cobra.Command, args []string) {
	badgerDb, err := badger.NewBadgerDB(".goray/cache/badger")
	if err != nil {
		log.Fatal(err)
		return
	}
	defer badgerDb.Close()

	dbClient := badger.NewBadgerClient(badgerDb)
	linkerRepo := dbClient.MakeLinkerRepo("goray")

	rfl := loader.NewFsLoader(new(syswrap.FsWrapper))
	linker := ropa.NewLinker(linkerRepo, radix.NewTree[*ropa.IndexedRegoFile]())

	bundle, err := rfl.LoadBundle("test.bundle")
	if err != nil {
		log.Fatal(err)
		return
	}

	rawRegoFiles := make([]*loader.RawRegoFile, 0)
	rawRegoFiles = append(rawRegoFiles, bundle.Files...)

	for _, pd := range policies {
		file, err := rfl.LoadRegoFile(pd.Path)
		if err != nil {
			log.Fatal(err)
			return
		}

		rawRegoFiles = append(rawRegoFiles, file)

		for _, path := range pd.Dependencies {
			depFile, err := rfl.LoadRegoFile(path)
			if err != nil {
				log.Fatal(err)
				return
			}

			rawRegoFiles = append(rawRegoFiles, depFile)
		}
	}

	indexedList := make([]*ropa.IndexedRegoFile, 0)

	for _, f := range rawRegoFiles {
		indexed, err := linker.Indexing(f)
		if err != nil {
			log.Fatal(err)
			return
		}

		indexedList = append(indexedList, indexed)
	}

	linkedList := make([]*ropa.LinkedRegoFile, len(indexedList))
	for i, f := range indexedList {
		linked, err := linker.Linking(f)
		if err != nil {
			log.Fatal(err)
			return
		}

		linkedList[i] = linked
	}

	for _, f := range linkedList {
		fmt.Println(f.Path, f.Parsed.Package.Path.String(), len(f.Dependencies))
	}
}

// func runRootCmd(cmd *cobra.Command, args []string) {
// db, err := badger.NewBadgerClient("tmp/badger")
// if err != nil {
// 	log.Fatal(err)
// }
// defer db.Close()

// opts := badger.DefaultOptions("tmp/badger")
// opts.Logger = nil
// db, err := badger.Open(opts)
// if err != nil {
// 	log.Fatal(err)
// }
// defer db.Close()
// c := CommandsRepository{db}
// if err := c.SetValue([]byte("hello"), []byte("world")); err != nil {
// 	log.Fatal(err)
// }
// v, err := c.GetValue([]byte("hello"))
// fmt.Println(string(v))
// if err != nil {
// 	log.Fatal(err)
// }

// loader := openpolicy.NewLoader(new(syswrap.FsClient))
// machine := openpolicy.NewMachine(loader, len(policies))

// b, err := loader.LoadBundle("bundle.tar.gz")
// if err != nil {
// 	log.Fatal(err)
// 	return
// }

// machine.RegisterBundle(b)

// for i, v := range policies {
// 	file, err := loader.LoadRegoFile(v.Path)
// 	if err != nil {
// 		log.Fatal(err)
// 		return
// 	}

// 	if err := machine.RegisterPolicy(&openpolicy.PolicyDescription{
// 		File:    file,
// 		Targets: policies[i].Target,
// 		Vendors: mapIncludeToVendorDesc(policies[i].Include),
// 	}); err != nil {
// 		log.Fatal(err)
// 		return
// 	}
// }

// compilers, err := machine.Compile()

// for _, v := range compilers[0].Modules {
// 	fmt.Println(v.Package)
// }

// fileMap, err := tmpGetFileAstAsMap("testdata/main.go")
// if err != nil {
// 	log.Fatal(err)
// 	return
// }

// var buf bytes.Buffer
// r := rego.New(
// 	rego.Query("data.goray"),
// 	rego.Compiler(compilers[0]),
// 	rego.Input(fileMap),
// 	rego.EnablePrintStatements(true),
// 	rego.PrintHook(topdown.NewPrintHook(&buf)),
// )

// rs, err := r.Eval(context.Background())
// if err != nil {
// 	log.Fatal(err)
// 	return
// }

// for _, result := range rs {
// 	for _, r := range result.Expressions {
// 		fmt.Println(r.Value)
// 	}
// }

// fmt.Println(buf.String())
// }

func tmpGetFileAstAsMap(filePath string) (map[string]interface{}, error) {
	fset := token.NewFileSet()

	f, err := parser.ParseFile(fset, filePath, nil, parser.ParseComments)
	if err != nil {
		return nil, err
	}

	pass := ason.NewSerPass(fset)
	fileAstJson := ason.SerializeFile(pass, f)

	jsonData, err := json.Marshal(fileAstJson)
	if err != nil {
		return nil, err
	}

	var fileMap map[string]interface{}
	if err := json.Unmarshal(jsonData, &fileMap); err != nil {
		return nil, err
	}

	return fileMap, nil
}
