package main

import (
	"fmt"
	"go/ast"
	"log"
	"reflect"
	"strings"

	"github.com/4rchr4y/go-compass"
	"golang.org/x/tools/go/analysis"
	"golang.org/x/tools/go/packages"
)

type MyWriter struct {
	data []byte
}

func (w *MyWriter) Write(p []byte) (int, error) {

	w.data = append(w.data, p...)
	return len(p), nil
}

var Analyzer = &analysis.Analyzer{
	Name: "doc",
	Doc:  "obj",
	Run: func(pass *analysis.Pass) (interface{}, error) {
		for _, file := range pass.Files {
			ast.Inspect(file, func(n ast.Node) bool {

				fn, ok := n.(*ast.FuncDecl)
				if !ok {
					return true
				}

				if fn.Doc == nil && strings.ToUpper(fn.Name.Name[:1]) == fn.Name.Name[:1] {
					pass.Reportf(fn.Pos(), "экспортируемая функция %q не имеет комментария GoDoc", fn.Name.Name)
				}
				return false
			})
		}
		return 1, nil
	},
}

func main() {
	cfg := &packages.Config{
		Mode: packages.LoadAllSyntax,
		// Dir:  "./...",
	}

	pkgs, err := packages.Load(cfg, ".")
	if err != nil {
		log.Fatalf("Error: %v", err)
	}

	for _, pkg := range pkgs {

		pass := &analysis.Pass{
			Analyzer:   Analyzer,
			Pkg:        pkg.Types,
			TypesInfo:  pkg.TypesInfo,
			Fset:       pkg.Fset,
			Files:      pkg.Syntax,
			Report:     func(diag analysis.Diagnostic) { log.Print(diag) },
			ResultOf:   make(map[*analysis.Analyzer]interface{}),
			OtherFiles: pkg.OtherFiles,
		}

		r, err := Analyzer.Run(pass)
		if err != nil {
			log.Printf(pkg.Name, err)
		}

		fmt.Println(r)

		// for _, f := range pkg.Syntax {
		// 	astFile := pkg.Fset.File(f.Pos())
		// 	if astFile != nil {
		// 		fmt.Println("0000 File:", astFile.Name())
		// 	}

		// 	for _, d := range f.Decls {
		// 		fmt.Println(reflect.TypeOf(d))
		// 	}
		// 	fmt.Println("-----", pkg.Name)
		// }

	}

	// ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	// defer cancel()
	// clientOptions := options.Client().ApplyURI("mongodb://ant:password@localhost:27017")

	// client, err := mongo.Connect(ctx, clientOptions)
	// if err != nil {
	// 	log.Fatal(err)
	// }

	// err = client.Ping(ctx, nil)
	// if err != nil {
	// 	log.Fatal(err)
	// }

	// fmt.Println("Successfully connected to MongoDB!")

	// db := client.Database("archant")
	// collection := db.Collection("someproject")

	// scanRepo := mongoScannerRepo.NewSnapshotRepository(collection)
	// scanService.Perform(ctx, "example/cmd", "github.com/g10z3r/archx")

	// myWriter := &MyWriter{}

	// logger := slog.New(slog.NewTextHandler(myWriter, nil))
	// fmt.Println(len(myWriter.data))
	// logger.Info("Test")
	// logger.Info("Test1")

	// fmt.Printf("Записанные логи:\n%s\n", myWriter.data)

	// compass.Run(context.Background())

	// clct := collector.DefaultCollector(
	// 	collector.WithTargetDir("example"),
	// )
	// if err := clct.Explore(); err != nil {
	// 	log.Fatal(err)
	// }

	// engine := compass.NewEngine(&compass.EngineConfig{
	// 	AnalyzerFactoryMap: getAnalyzers(),
	// 	ModuleName:         clct.GetInfo().ModuleName,
	// })

	// var wg sync.WaitGroup

	// eventCh := make(chan event.Event)
	// unsubscribeCh := compass.Subscribe(eventCh)

	// wg.Add(1)
	// go func() {
	// 	defer wg.Done()
	// 	for {
	// 		select {
	// 		case e := <-eventCh:
	// 			switch ev := e.(type) {
	// 			case *event.PackageFormedEvent:
	// 				jsonData, err := json.Marshal(ev.Package)
	// 				if err != nil {
	// 					log.Fatal(err)
	// 				}

	// 				fmt.Println(string(jsonData))

	// 			default:
	// 				fmt.Printf("Unknown event type: %s\n", e.Name())
	// 			}
	// 		case <-unsubscribeCh:
	// 			return
	// 		}
	// 	}
	// }()

	// for _, p := range clct.GetAllPackageDirs() {
	// 	data, err := engine.ParseDir(p)
	// 	if err != nil {

	// 	}

	// 	for _, pkg := range data {
	// 		jsonData, err := json.Marshal(pkg)
	// 		if err != nil {
	// 			log.Fatal(err)
	// 		}

	// 		fmt.Println("\n", string(jsonData))
	// 	}

	// }

	// compass := compass.New(&compass.Config{
	// 	RootDir:     ".",
	// 	TargetDir:   "",
	// 	IgnoredList: compass.DefaultIgnoredList,
	// 	Group:       getAnalyzers(),
	// })

	// if err := compass.Scan(); err != nil {
	// 	fmt.Println(err)
	// }
}

// TODO: tmp func
func getAnalyzers() compass.PickerFactoryGroup {
	return compass.PickerFactoryGroup{
		reflect.TypeOf(new(ast.ImportSpec)): compass.NewImportSpecPicker,
		reflect.TypeOf(new(ast.FuncDecl)):   compass.NewFuncDeclPicker,
		reflect.TypeOf(new(ast.StructType)): compass.NewStructTypePicker,
		reflect.TypeOf(new(ast.FuncType)):   compass.NewFuncTypePicker,
	}
}
