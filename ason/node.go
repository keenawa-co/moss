package ason

// NodeType constants define the string representation of various AST node types.
const (
	NodeTypeInvalid        = "Invalid"
	NodeTypeFile           = "File"
	NodeTypeComment        = "Comment"
	NodeTypeCommentGroup   = "CommentGroup"
	NodeTypeIdent          = "Ident"
	NodeTypeBasicLit       = "BasicLit"
	NodeTypeCompositeLit   = "CompositeLit"
	NodeTypeValueSpec      = "ValueSpec"
	NodeTypeGenDecl        = "GenDecl"
	NodeTypeField          = "Field"
	NodeTypeFieldList      = "FieldList"
	NodeTypeEllipsis       = "Ellipsis"
	NodeTypeBadExpr        = "BadExpr"
	NodeTypeParenExpr      = "ParenExpr"
	NodeTypeSelectorExpr   = "SelectorExpr"
	NodeTypeIndexExpr      = "IndexExpr"
	NodeTypeIndexListExpr  = "IndexListExpr"
	NodeTypeSliceExpr      = "SliceExpr"
	NodeTypeTypeAssertExpr = "TypeAssertExpr"
	NodeTypeCallExpr       = "CallExpr"
	NodeTypeStarExpr       = "StarExpr"
	NodeTypeUnaryExpr      = "UnaryExpr"
	NodeTypeBinaryExpr     = "BinaryExpr"
	NodeTypeKeyValueExpr   = "KeyValueExpr"
)

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
	Ref  uint   `json:"_ref"`
	Type string `json:"_type"`
}

type File struct {
	Doc   *CommentGroup `json:"Doc,omitempty"`  // associated documentation; or empty
	Name  *Ident        `json:"Name,omitempty"` // package name
	Decls []Decl        `json:"Decl"`           // top-level declarations

	Loc  *Loc // start and end of entire file
	Size int  // file size in bytes

	// Scope              *Scope          // package scope (this file only)
	// Imports            []*ImportSpec   // imports in this file
	// Unresolved         []*Ident        // unresolved identifiers in this file
	Package   Pos             `json:"Package,omitempty"`   // position of "package" keyword
	Comments  []*CommentGroup `json:"Comments,omitempty"`  // list of all comments in the source file
	GoVersion string          `json:"GoVersion,omitempty"` // minimum Go version required by go:build or +build directives

	Node
}

func (*File) asonNode() {}

type (
	Comment struct {
		Slash Pos    // position of "/" starting the comment
		Text  string // comment text (excluding '\n' for //-style comments)

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

// ----------------- Expressions ----------------- //

type Field struct {
	Doc     *CommentGroup // associated documentation; or nil
	Names   []*Ident      // field/method/(type) parameter names; or nil
	Type    Expr          // field/method/parameter type; or nil
	Tag     *BasicLit     // field tag; or nil
	Comment *CommentGroup // line comments; or nil

	Node
}

// A FieldList represents a list of Fields, enclosed by parentheses,
// curly braces, or square brackets.
type FieldList struct {
	Opening Pos      // position of opening parenthesis/brace/bracket, if any
	List    []*Field // field list; or nil
	Closing Pos      // position of closing parenthesis/brace/bracket, if any

	Node
}

func (*Field) asonNode()     {}
func (*FieldList) asonNode() {}

type (
	// An Ident node represents an identifier.
	Ident struct {
		NamePos Pos    // identifier position
		Name    string // identifier name

		Node
	}

	// A BasicLit node represents a literal of basic type.
	BasicLit struct {
		ValuePos Pos    // literal position
		Kind     string // token.INT, token.FLOAT, token.IMAG, token.CHAR, or token.STRING
		Value    string // literal string; e.g. 42, 0x7f, 3.14, 1e-9, 2.4i, 'a', '\x7f', "foo" or `\m\n\o`

		Node
	}

	// A CompositeLit node represents a composite literal.
	CompositeLit struct {
		Type       Expr   // literal type; or nil
		Lbrace     Pos    // position of "{"
		Elts       []Expr // list of composite elements; or nil
		Rbrace     Pos    // position of "}"
		Incomplete bool   // true if (source) expressions are missing in the Elts list

		Node
	}

	// An Ellipsis node stands for the "..." type in a
	// parameter list or the "..." length in an array type.
	Ellipsis struct {
		Ellipsis Pos  // position of "..."
		Elt      Expr // ellipsis element type (parameter lists only); or nil

		Node
	}

	// A BadExpr node is a placeholder for an expression containing
	// syntax errors for which a correct expression node cannot be
	// created.
	BadExpr struct {
		Loc *Loc

		Node
	}

	// A ParenExpr node represents a parenthesized expression.
	ParenExpr struct {
		Lparen Pos  // position of "("
		X      Expr // parenthesized expression
		Rparen Pos  // position of ")"

		Node
	}

	// A SelectorExpr node represents an expression followed by a selector.
	SelectorExpr struct {
		X   Expr   // expression
		Sel *Ident // field selector

		Node
	}

	// An IndexExpr node represents an expression followed by an index.
	IndexExpr struct {
		X      Expr // expression
		Lbrack Pos  // position of "["
		Index  Expr // index expression
		Rbrack Pos  // position of "]"

		Node
	}

	// An IndexListExpr node represents an expression followed by multiple indices.
	IndexListExpr struct {
		X       Expr   // expression
		Lbrack  Pos    // position of "["
		Indices []Expr // index expressions
		Rbrack  Pos    // position of "]"

		Node
	}

	// A SliceExpr node represents an expression followed by slice indices.
	SliceExpr struct {
		X      Expr // expression
		Lbrack Pos  // position of "["
		Low    Expr // begin of slice range; or nil
		High   Expr // end of slice range; or nil
		Max    Expr // maximum capacity of slice; or nil
		Slice3 bool // true if 3-index slice (2 colons present)
		Rbrack Pos  // position of "]"

		Node
	}

	// A TypeAssertExpr node represents an expression followed by a type assertion.
	TypeAssertExpr struct {
		X      Expr // expression
		Lparen Pos  // position of "("
		Type   Expr // asserted type; nil means type switch X.(type)
		Rparen Pos  // position of ")"

		Node
	}

	// A CallExpr node represents an expression followed by an argument list.
	CallExpr struct {
		Fun      Expr   // function expression
		Lparen   Pos    // position of "("
		Args     []Expr // function arguments; or nil
		Ellipsis Pos    // position of "..." (token.NoPos if there is no "...")
		Rparen   Pos    // position of ")"

		Node
	}

	// A StarExpr node represents an expression of the form "*" Expression.
	// Semantically it could be a unary "*" expression, or a pointer type.
	StarExpr struct {
		Star Pos  // position of "*"
		X    Expr // operand

		Node
	}

	// A UnaryExpr node represents a unary expression.
	// Unary "*" expressions are represented via StarExpr nodes.
	//
	UnaryExpr struct {
		OpPos Pos    // position of Op
		Op    string // operator (token)
		X     Expr   // operand

		Node
	}

	// A BinaryExpr node represents a binary expression.
	BinaryExpr struct {
		X     Expr   // left operand
		OpPos Pos    // position of Op
		Op    string // operator (token)
		Y     Expr   // right operand

		Node
	}

	// A KeyValueExpr node represents (key : value) pairs
	// in composite literals.
	//
	KeyValueExpr struct {
		Key   Expr
		Colon Pos // position of ":"
		Value Expr

		Node
	}
)

func (*Ident) asonNode()          {}
func (*BasicLit) asonNode()       {}
func (*BadExpr) asonNode()        {}
func (*Ellipsis) asonNode()       {}
func (*CompositeLit) asonNode()   {}
func (*ParenExpr) asonNode()      {}
func (*IndexExpr) asonNode()      {}
func (*IndexListExpr) asonNode()  {}
func (*SliceExpr) asonNode()      {}
func (*TypeAssertExpr) asonNode() {}
func (*CallExpr) asonNode()       {}
func (*StarExpr) asonNode()       {}
func (*UnaryExpr) asonNode()      {}
func (*BinaryExpr) asonNode()     {}
func (*KeyValueExpr) asonNode()   {}

func (*Ident) exprNode()          {}
func (*BasicLit) exprNode()       {}
func (*BadExpr) exprNode()        {}
func (*Ellipsis) exprNode()       {}
func (*CompositeLit) exprNode()   {}
func (*ParenExpr) exprNode()      {}
func (*IndexExpr) exprNode()      {}
func (*IndexListExpr) exprNode()  {}
func (*SliceExpr) exprNode()      {}
func (*TypeAssertExpr) exprNode() {}
func (*CallExpr) exprNode()       {}
func (*StarExpr) exprNode()       {}
func (*UnaryExpr) exprNode()      {}
func (*BinaryExpr) exprNode()     {}
func (*KeyValueExpr) exprNode()   {}

// ----------------- Types ----------------- //

type ()

// ----------------- Specifications ----------------- //

type (
	ValueSpec struct {
		Loc    *Loc
		Names  []*Ident
		Values []Expr

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

		Node
	}
)

func (*GenDecl) asonNode() {}

func (*GenDecl) declNode() {}
