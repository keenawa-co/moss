package obj

import (
	"fmt"
	"go/ast"
	"go/token"
	"reflect"
	"unicode"
)

type (
	FuncObjParam struct {
		Type  string
		Usage int
	}

	FuncDeclObj struct {
		Name           string
		Receiver       *string
		FieldAccess    map[string]int
		Params         map[string]*FuncObjParam
		Dependencies   map[string]*DependencyObj
		Visibility     bool
		Arity          int
		ReturnCount    int
		Recursive      bool
		HasSideEffects bool
	}
)

func (o *FuncDeclObj) Type() string {
	return "func"
}

func (o *FuncDeclObj) AddDependency(importIndex int, element string) {
	if _, exists := o.Dependencies[element]; !exists {
		o.Dependencies[element] = &DependencyObj{
			ImportIndex: importIndex,
			Usage:       1,
		}

		return
	}

	o.Dependencies[element].Usage++
}

func NewFuncDeclObj(fset *token.FileSet, res *ast.FuncDecl, params map[string]*FuncObjParam, initDeps map[string]*DependencyObj, receiver *ast.Ident) *FuncDeclObj {
	funcDeclObj := new(FuncDeclObj)

	funcDeclObj.Name = res.Name.Name
	funcDeclObj.Dependencies = initDeps
	funcDeclObj.Params = params
	funcDeclObj.Visibility = unicode.IsUpper(rune(res.Name.Name[0]))
	funcDeclObj.Arity = len(params)

	if receiver != nil {
		// Adding a `$` sign to distinguish between method names and regular function names.
		// To use function names as declaration map keys in a file object
		funcDeclObj.Name = "$" + funcDeclObj.Name
		funcDeclObj.Receiver = &receiver.Name
		funcDeclObj.FieldAccess = make(map[string]int)
	}

	return funcDeclObj
}

type (
	// TODO: get rid of this structure
	FuncFieldObj struct {
		Name string
		Type string
	}

	FuncTypeObj struct {
		Params  []*StructFieldObj         // TODO: convert it into a map
		Results map[string]*FuncFieldObj  // TODO: Not implemented
		Deps    map[string]*DependencyObj // TODO: Not implemented
	}
)

func NewFuncTypeObj(fset *token.FileSet, node ast.Node) (*FuncTypeObj, error) {
	ts, ok := node.(*ast.TypeSpec)
	if !ok {
		return nil, fmt.Errorf("node is not a TypeSpec: %s", reflect.TypeOf(node))
	}

	funcType, ok := ts.Type.(*ast.FuncType)
	if !ok {
		return nil, fmt.Errorf("node is not a FuncType: %s", reflect.TypeOf(node))
	}

	extrParamsData, err := extractFieldMap(fset, funcType.Params.List)
	if err != nil {
		return nil, fmt.Errorf("failed to extract func field map: %w", err)
	}

	return &FuncTypeObj{
		Params: extrParamsData.fieldsSet,
	}, nil
}
