package obj

const NoPos int = 0

type ObjKind int

const (
	Bad ObjKind = iota // for error handling
	Pkg                // package
	Imp                // import
	Con                // constant
	Typ                // type
	Var                // variable
	Fun                // function or method
	Lbl                // label
)

var objKindStrings = [...]string{
	Bad: "bad",
	Pkg: "package",
	Imp: "import",
	Con: "const",
	Typ: "type",
	Var: "var",
	Fun: "func",
	Lbl: "label",
}

func (kind ObjKind) String() string { return objKindStrings[kind] }

type Object interface {
	Kind() ObjKind
	IsValid() bool
	IsExported() bool
}
