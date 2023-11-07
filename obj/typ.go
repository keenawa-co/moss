package obj

type AstTyp uint

const (
	ImportSpec AstTyp = iota + 1
	TypeScpec
	FuncDecl
	StructType
	FuncType
	InterfaceType
)

const (
	UndefinedString     = "undefined"
	ImportSpecString    = "import_spec"
	TypeScpecString     = "type_scpec"
	FuncDeclString      = "func_decl"
	StructTypeString    = "struct_type"
	FuncTypeString      = "func_type"
	InterfaceTypeString = "interface_type"
)

func (typ AstTyp) String() string {
	switch typ {
	case ImportSpec:
		return ImportSpecString
	case TypeScpec:
		return TypeScpecString
	case FuncDecl:
		return FuncDeclString
	case StructType:
		return StructTypeString
	case FuncType:
		return FuncTypeString
	case InterfaceType:
		return InterfaceTypeString
	}

	return UndefinedString
}
