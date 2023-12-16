package goray

import data.go.types


is_function_used(decls, pkg_name, func_name) {
    some i
    decl := decls[i]
    decl._type == "FuncDecl"
    decl.Body.List[_].X.Fun.X.Name == pkg_name     
    decl.Body.List[_].X.Fun.Sel.Name == func_name           
}

warning[res] {
    is_function_used(input.Decls, "fmt", "Println")
    res := {
        "msg": "do not use fmt.Println",
        "sev": types.func_decl
    }
}