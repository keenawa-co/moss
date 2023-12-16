package goray

# METADATA
# title: Allow Ones
# description: |
#  The 'allow' rule...
#  Is about allowing things.
#  Not denying them.
# authors:
# - Jane Doe <jane@example.com>

# fail[res] {
#     input.Kind == "ExprStmt"
#     input.Node.X.Fun.X.Name == "fmt"
#     input.Node.X.Fun.Sel.Name == "Sprintln"

#     print("test")

#     res := {
#         "msg": "do1111 not use fmt.Sprintln",
#         "pos": input.Node.X.Fun.X.NamePos,
#         "sev": "ERROR",
#     }
# }

warning[res] {
    input.Decls[_]._type == "FuncDecl2"
    #input.Decls.Body._type == "ExprStmt"
    # input.Decls.Body.X.Fun.X.Name == "fmt"
    # input.Decls.Body.X.Fun.Sel.Name == "Println"

    print("input")

    res := {
        "msg": "do not use fmt.Println",
        "pos": "input.Node.X.Fun.X.NamePos",
        "sev": "WARN",
    }
}