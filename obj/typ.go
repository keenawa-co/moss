package obj

type AstTyp uint

const (
	ImportSpec AstTyp = iota + 1
	TypeSpec
	FuncDecl
	StructType
	FuncType
	InterfaceType
)

const (
	UndefinedString     = "undefined"
	ImportSpecString    = "import_spec"
	TypeSpecString      = "type_spec"
	FuncDeclString      = "func_decl"
	StructTypeString    = "struct_type"
	FuncTypeString      = "func_type"
	InterfaceTypeString = "interface_type"
)

func (typ AstTyp) String() string {
	switch typ {
	case ImportSpec:
		return ImportSpecString
	case TypeSpec:
		return TypeSpecString
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
