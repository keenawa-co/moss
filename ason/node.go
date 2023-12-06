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
	NodeTypeBadStmt        = "BadStmt"
	NodeTypeDeclStmt       = "DeclStmt"
	NodeTypeEmptyStmt      = "EmptyStmt"
	NodeTypeLabeledStmt    = "LabeledStmt"
	NodeTypeExprStmt       = "ExprStmt"
	NodeTypeSendStmt       = "SendStmt"
	NodeTypeIncDecStmt     = "IncDecStmt"
	NodeTypeAssignStmt     = "AssignStmt"
	NodeTypeGoStmt         = "GoStmt"
	NodeTypeDeferStmt      = "DeferStmt"
	NodeTypeReturnStmt     = "ReturnStmt"
	NodeTypeBranchStmt     = "BranchStmt"
	NodeTypeBlockStmt      = "BlockStmt"
	NodeTypeIfStmt         = "IfStmt"
	NodeTypeCaseClause     = "CaseClause"
	NodeTypeSwitchStmt     = "SwitchStmt"
	NodeTypeTypeSwitchStmt = "TypeSwitchStmt"
	NodeTypeCommClause     = "CommClause"
	NodeTypeSelectStmt     = "SelectStmt"
	NodeTypeForStmt        = "ForStmt"
	NodeTypeRangeStmt      = "RangeStmt"
)

type Ason interface {
	asonNode()
}

type Expr interface {
	Ason
	exprNode()
}

type Spec interface {
	Ason
	specNode()
}
type Stmt interface {
	Ason
	stmtNode()
}

type Decl interface {
	Ason
	declNode()
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

// ----------------- Statements ----------------- //

type (
	// A BadStmt node is a placeholder for statements containing
	// syntax errors for which no correct statement nodes can be created.
	BadStmt struct {
		From, To Pos // position range of bad statement
	}

	// A DeclStmt node represents a declaration in a statement list.
	DeclStmt struct {
		Decl Decl // *GenDecl with CONST, TYPE, or VAR token
	}

	// An EmptyStmt node represents an empty statement.
	// The "position" of the empty statement is the position
	// of the immediately following (explicit or implicit) semicolon.
	EmptyStmt struct {
		Semicolon Pos  // position of following ";"
		Implicit  bool // if set, ";" was omitted in the source
	}

	// A LabeledStmt node represents a labeled statement.
	LabeledStmt struct {
		Label *Ident
		Colon Pos // position of ":"
		Stmt  Stmt
	}

	// An ExprStmt node represents a (stand-alone) expression in a statement list.
	ExprStmt struct {
		X Expr // expression
	}

	// A SendStmt node represents a send statement.
	SendStmt struct {
		Chan  Expr
		Arrow Pos // position of "<-"
		Value Expr
	}

	// An IncDecStmt node represents an increment or decrement statement.
	IncDecStmt struct {
		X      Expr
		TokPos Pos    // position of Tok
		Tok    string // INC or DEC
	}

	// An AssignStmt node represents an assignment or a short variable declaration.
	AssignStmt struct {
		Lhs    []Expr
		TokPos Pos    // position of Tok
		Tok    string // assignment token, DEFINE
		Rhs    []Expr
	}

	// A GoStmt node represents a go statement.
	GoStmt struct {
		Go   Pos // position of "go" keyword
		Call *CallExpr
	}

	// A DeferStmt node represents a defer statement.
	DeferStmt struct {
		Defer Pos // position of "defer" keyword
		Call  *CallExpr
	}

	// A ReturnStmt node represents a return statement.
	ReturnStmt struct {
		Return  Pos    // position of "return" keyword
		Results []Expr // result expressions; or nil
	}

	// A BranchStmt node represents a break, continue, goto,
	// or fallthrough statement.
	BranchStmt struct {
		TokPos Pos    // position of Tok
		Tok    string // keyword token (BREAK, CONTINUE, GOTO, FALLTHROUGH)
		Label  *Ident // label name; or nil
	}

	// A BlockStmt node represents a braced statement list.
	BlockStmt struct {
		Lbrace Pos // position of "{"
		List   []Stmt
		Rbrace Pos // position of "}", if any (may be absent due to syntax error)
	}

	// An IfStmt node represents an if statement.
	IfStmt struct {
		If   Pos  // position of "if" keyword
		Init Stmt // initialization statement; or nil
		Cond Expr // condition
		Body *BlockStmt
		Else Stmt // else branch; or nil
	}

	// A CaseClause represents a case of an expression or type switch statement.
	CaseClause struct {
		Case  Pos    // position of "case" or "default" keyword
		List  []Expr // list of expressions or types; nil means default case
		Colon Pos    // position of ":"
		Body  []Stmt // statement list; or nil
	}

	// A SwitchStmt node represents an expression switch statement.
	SwitchStmt struct {
		Switch Pos        // position of "switch" keyword
		Init   Stmt       // initialization statement; or nil
		Tag    Expr       // tag expression; or nil
		Body   *BlockStmt // CaseClauses only
	}

	// A TypeSwitchStmt node represents a type switch statement.
	TypeSwitchStmt struct {
		Switch Pos        // position of "switch" keyword
		Init   Stmt       // initialization statement; or nil
		Assign Stmt       // x := y.(type) or y.(type)
		Body   *BlockStmt // CaseClauses only
	}

	// A CommClause node represents a case of a select statement.
	CommClause struct {
		Case  Pos    // position of "case" or "default" keyword
		Comm  Stmt   // send or receive statement; nil means default case
		Colon Pos    // position of ":"
		Body  []Stmt // statement list; or nil
	}

	// A SelectStmt node represents a select statement.
	SelectStmt struct {
		Select Pos        // position of "select" keyword
		Body   *BlockStmt // CommClauses only
	}

	// A ForStmt represents a for statement.
	ForStmt struct {
		For  Pos  // position of "for" keyword
		Init Stmt // initialization statement; or nil
		Cond Expr // condition; or nil
		Post Stmt // post iteration statement; or nil
		Body *BlockStmt
	}

	// A RangeStmt represents a for statement with a range clause.
	RangeStmt struct {
		For        Pos    // position of "for" keyword
		Key, Value Expr   // Key, Value may be nil
		TokPos     Pos    // position of Tok; invalid if Key == nil
		Tok        string // ILLEGAL if Key == nil, ASSIGN, DEFINE
		Range      Pos    // position of "range" keyword
		X          Expr   // value to range over
		Body       *BlockStmt
	}
)

func (*BadStmt) asonNode()        {}
func (*DeclStmt) asonNode()       {}
func (*EmptyStmt) asonNode()      {}
func (*LabeledStmt) asonNode()    {}
func (*ExprStmt) asonNode()       {}
func (*SendStmt) asonNode()       {}
func (*IncDecStmt) asonNode()     {}
func (*AssignStmt) asonNode()     {}
func (*GoStmt) asonNode()         {}
func (*DeferStmt) asonNode()      {}
func (*ReturnStmt) asonNode()     {}
func (*BranchStmt) asonNode()     {}
func (*BlockStmt) asonNode()      {}
func (*IfStmt) asonNode()         {}
func (*CaseClause) asonNode()     {}
func (*SwitchStmt) asonNode()     {}
func (*TypeSwitchStmt) asonNode() {}
func (*CommClause) asonNode()     {}
func (*SelectStmt) asonNode()     {}
func (*ForStmt) asonNode()        {}
func (*RangeStmt) asonNode()      {}

func (*BadStmt) stmtNode()        {}
func (*DeclStmt) stmtNode()       {}
func (*EmptyStmt) stmtNode()      {}
func (*LabeledStmt) stmtNode()    {}
func (*ExprStmt) stmtNode()       {}
func (*SendStmt) stmtNode()       {}
func (*IncDecStmt) stmtNode()     {}
func (*AssignStmt) stmtNode()     {}
func (*GoStmt) stmtNode()         {}
func (*DeferStmt) stmtNode()      {}
func (*ReturnStmt) stmtNode()     {}
func (*BranchStmt) stmtNode()     {}
func (*BlockStmt) stmtNode()      {}
func (*IfStmt) stmtNode()         {}
func (*CaseClause) stmtNode()     {}
func (*SwitchStmt) stmtNode()     {}
func (*TypeSwitchStmt) stmtNode() {}
func (*CommClause) stmtNode()     {}
func (*SelectStmt) stmtNode()     {}
func (*ForStmt) stmtNode()        {}
func (*RangeStmt) stmtNode()      {}

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
