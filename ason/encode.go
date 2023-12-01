package ason

import (
	"go/ast"
)

func MarshalIdent(input *ast.Ident) *IdentMod {
	return &IdentMod{
		Pos:        input.Pos(),
		End:        input.End(),
		Name:       input.Name,
		Obj:        MarshalObject(input.Obj),
		IsExported: input.IsExported(),
	}
}

func MarshalObject(input *ast.Object) *ObjectMod {
	if input == nil {
		return nil
	}

	output := &ObjectMod{
		Kind: input.Kind.String(),
		Name: input.Name,
	}

	if input.Decl != nil {
		// TODO: to use MarshalDecl
	}

	return output
}
