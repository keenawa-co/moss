package main

import "go/ast"

/*
Hello World
*/
var test2 = "test2Value"

var (
	// Some doc here
	test3 = "test3Value"
	test4 = "test4Value"
)

// var testArray = [5]int{1, 2, 3, 4, 5}

type serConf int

const (
	CACHE_REF serConf = iota << 2
	FILE_SCOPE
	PKG_SCOPE
	IDENT_OBJ
	LOC
)

const (
	CACHE_REF = 1
	FILE_SCOPE
	PKG_SCOPE
	IDENT_OBJ
	LOC
)

func SerializePackage(pass *serPass, input *ast.Package) *Package {
	var scope *Scope
	if pass.conf[PKG_SCOPE] != nil && input.Scope != nil {
		scope = SerializeScope(pass, input.Scope)
	}

	return &Package{
		Name:  input.Name,
		Scope: scope,
		// cant use SerializeMap func, because `*ast.Object` does not satisfy the interface `ast.Node`.
		Imports: serializeImports(pass, input.Imports),
		Files:   SerializeMap(pass, input.Files, SerializeFile),
	}
}

func serializeImports(pass *serPass, inputMap map[string]*ast.Object) map[string]*Object {
	result := make(map[string]*Object, len(inputMap))
	for k, v := range inputMap {
		result[k] = SerializeObject(pass, v)
	}

	return result
}
