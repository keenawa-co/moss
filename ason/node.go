package ason

type Ason interface {
	asonNode()
}

type Spec interface {
	Ason
	specNode()
}

type Decl interface {
	Ason
	declNode()
}

type Expr interface {
	Ason
	exprNode()
}

type Node struct {
	Ref  uint   // `json:"-"`
	Type string `json:"_type"`
}

type File struct {
	Doc   *CommentGroup `json:"Doc,omitempty"`  // associated documentation; or empty
	Name  *Ident        `json:"Name,omitempty"` // package name
	Decls []Decl        `json:"Decl"`           // top-level declarations

	Loc *Loc // start and end of entire file

	// Scope              *Scope          // package scope (this file only)
	// Imports            []*ImportSpec   // imports in this file
	// Unresolved         []*Ident        // unresolved identifiers in this file
	Package   Pos             `json:"Package,omitempty"`   // position of "package" keyword
	Comments  []*CommentGroup `json:"Comments,omitempty"`  // list of all comments in the source file
	GoVersion string          `json:"GoVersion,omitempty"` // minimum Go version required by //go:build or // +build directives

	Node
}

func (*File) asonNode() {}

type (
	Comment struct {
		Slash Pos    `json:"Slash"`          // position of "/" starting the comment
		Text  string `json:"Text,omitempty"` // comment text (excluding '\n' for //-style comments)

		Node
	}

	// A CommentGroup represents a sequence of comments
	// with no other tokens and no empty lines between.
	CommentGroup struct {
		List []*Comment

		Node
	}
)

func (*Comment) asonNode()      {}
func (*CommentGroup) asonNode() {}

// --------------------------------------------
// Expr

type (
	Ident struct {
		Loc     *Loc   `json:"Loc"`
		NamePos Pos    `json:"NamePos"`
		Name    string `json:"Name,omitempty"`

		Node
	}

	BasicLit struct {
		Loc      *Loc   `json:"Loc"`
		ValuePos Pos    `json:"ValuePos"`
		Kind     string `json:"Kind"`
		Value    string `json:"Value"`

		Node
	}
)

func (*Ident) asonNode()    {}
func (*BasicLit) asonNode() {}

func (*Ident) exprNode()    {}
func (*BasicLit) exprNode() {}

// --------------------------------------------
// Spec

type (
	ValueSpec struct {
		Loc    *Loc     `json:"Loc"`
		Names  []*Ident `json:"Names,omitempty"`
		Values []Expr   `json:"Values,omitempty"`

		Node
	}
)

func (*ValueSpec) asonNode() {}

func (*ValueSpec) specNode() {}

// --------------------------------------------
// Decl

type (
	GenDecl struct {
		Loc      *Loc   `json:"Loc"`
		TokenPos Pos    `json:"TokenPos"`
		Lparen   Pos    `json:"Lparen"`
		Rparen   Pos    `json:"Rparen"`
		Tok      string `json:"Tok"`
		Specs    []Spec `json:"Specs"`
	}
)

func (*GenDecl) asonNode() {}

func (*GenDecl) declNode() {}
