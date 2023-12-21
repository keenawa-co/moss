# METADATA
# scope: document
# description: A set of rules that determines if x is allowed.
package goray

import data.go.ast.types

is_function_used(decls, pkg_name, func_name) {
    some i
    decl := decls[i]
    decl._type == types.FuncDecl # "FuncDecl"
    decl.Body.List[_].X.Fun.X.Name == pkg_name     
    decl.Body.List[_].X.Fun.Sel.Name == func_name           
}

warning[res] {
    is_function_used(input.Decls, "fmt", "Println")
    res := {
        "msg": "do not use fmt.Println",
        "sev": "something.some_test_var"
    }
}