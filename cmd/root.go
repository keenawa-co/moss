package main

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

var rootCmd = &cobra.Command{
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
		Path:    "builtin/opa/policy/r1.rego",
		Target:  []string{"testdata/main.go"},
		Include: []string{"testdata/test.rego"},
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
	badgerDb, err := badger.NewBadgerDB("tmp/badger")
	if err != nil {
		log.Fatal(err)
		return
	}
	defer badgerDb.Close()

	dbClient := badger.NewBadgerClient(badgerDb)
	policyRepo := dbClient.MakePolicyRepo("goray")

	loader := loader.NewFsLoader(new(syswrap.FsClient))

	bundle, err := loader.LoadBundle("bundle.tar.gz")
	if err != nil {
		log.Fatal(err)
		return
	}

	for _, f := range bundle.Files {
		if err := policyRepo.Store(f); err != nil {
			log.Fatal(err)
			return
		}
	}

	linker := ropa.NewLinker(policyRepo, radix.NewTree[*ropa.LinkerOutput]())

	for _, f := range bundle.Files {
		fileMeta, err := linker.ProcessRegoFileMeta(f)
		if err != nil {
			log.Fatal(err)
			return
		}

		input := &ropa.LinkerInput{
			Parent:  f,
			Imports: fileMeta.Dependencies,
		}

		output, err := linker.Link(input)
		if err != nil {
			log.Fatal(err)
			return
		}

		fmt.Println(f.Path, len(output.Imports))
	}

	// f, err := policyRepo.Load("go/ast/kinds.rego")
	// if err != nil {
	// 	log.Fatal(err)
	// 	return
	// }

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

// type Command struct {
// 	key   []byte
// 	value []byte
// }

// type Repository interface {
// 	GetAll() ([]Command, error) // TODO ask about this
// 	GetValue([]byte) []byte
// 	SetValue(key, value []byte)
// 	EditValue([]byte)
// 	DeleteValue([]byte) bool // might change this
// }

// type CommandsRepository struct {
// 	db *badger.DB
// }

// func (c *CommandsRepository) GetAll() ([]Command, error) {
// 	var cmds []Command
// 	err := c.db.View(func(txn *badger.Txn) error {
// 		opts := badger.DefaultIteratorOptions
// 		opts.PrefetchSize = 10
// 		it := txn.NewIterator(opts)
// 		defer it.Close()
// 		for it.Rewind(); it.Valid(); it.Next() {
// 			item := it.Item()
// 			k := item.Key()
// 			err := item.Value(func(v []byte) error {
// 				cmds = append(cmds, Command{key: k, value: v})
// 				return nil
// 			})
// 			if err != nil {
// 				return err
// 			}
// 		}
// 		return nil
// 	})
// 	if err != nil {
// 		return nil, err
// 	}
// 	return cmds, nil
// }

// func (c *CommandsRepository) SetValue(k, v []byte) error {
// 	err := c.db.Update(func(txn *badger.Txn) error {
// 		err := txn.Set(k, v)
// 		return err
// 	})
// 	return err
// }

// func (c *CommandsRepository) GetValue(k []byte) ([]byte, error) {
// 	var v []byte
// 	err := c.db.View(func(txn *badger.Txn) error {
// 		i, err := txn.Get(k)
// 		if err != nil {
// 			v = []byte("this command does not exist")
// 			return nil
// 		}
// 		v, err = i.ValueCopy(v)
// 		return err
// 	})
// 	return v, err
// }

// func (c *CommandsRepository) DeleteValue(k []byte) error {
// 	err := c.db.Update(func(txn *badger.Txn) error {
// 		err := txn.Delete(k)
// 		return err
// 	})
// 	return err
// }
