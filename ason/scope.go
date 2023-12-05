package ason

import (
	"go/token"
	"strconv"
)

func _GOARCH() int {
	return strconv.IntSize
}

type (
	Pos           interface{ pos() }
	NoPos         int
	PosCompressed struct {
		Filename string
		Line     int // line number, starting at 1
	}
	Position struct {
		_        [0]int
		Filename string
		Offset   int // offset, starting at 0
		Line     int // line number, starting at 1
		Column   int // column number, starting at 1 (byte count)
	}
)

func (*NoPos) pos()         {}
func (*PosCompressed) pos() {}
func (*Position) pos()      {}

type Loc struct {
	_     [0]int
	Start Pos `json:"Start"`
	End   Pos `json:"End"`
}

var tokens = map[string]token.Token{
	"ILLEGAL": token.ILLEGAL,

	"EOF":     token.EOF,
	"COMMENT": token.COMMENT,

	"IDENT":  token.IDENT,
	"INT":    token.INT,
	"FLOAT":  token.FLOAT,
	"IMAG":   token.IMAG,
	"CHAR":   token.CHAR,
	"STRING": token.STRING,

	"+": token.ADD,
	"-": token.SUB,
	"*": token.MUL,
	"/": token.QUO,
	"%": token.REM,

	"&":  token.AND,
	"|":  token.OR,
	"^":  token.XOR,
	"<<": token.SHL,
	">>": token.SHR,
	"&^": token.AND_NOT,

	"+=": token.ADD_ASSIGN,
	"-=": token.SUB_ASSIGN,
	"*=": token.MUL_ASSIGN,
	"/=": token.QUO_ASSIGN,
	"%=": token.REM_ASSIGN,

	"&=":  token.AND_ASSIGN,
	"|=":  token.OR_ASSIGN,
	"^=":  token.XOR_ASSIGN,
	"<<=": token.SHL_ASSIGN,
	">>=": token.SHR_ASSIGN,
	"&^=": token.AND_NOT_ASSIGN,

	"&&": token.LAND,
	"||": token.LOR,
	"<-": token.ARROW,
	"++": token.INC,
	"--": token.DEC,

	"==": token.EQL,
	"<":  token.LSS,
	">":  token.GTR,
	"=":  token.ASSIGN,
	"!":  token.NOT,

	"!=":  token.NEQ,
	"<=":  token.LEQ,
	">=":  token.GEQ,
	":=":  token.DEFINE,
	"...": token.ELLIPSIS,

	"(": token.LPAREN,
	"[": token.LBRACK,
	"{": token.LBRACE,
	",": token.COMMA,
	".": token.PERIOD,

	")": token.RPAREN,
	"]": token.RBRACK,
	"}": token.RBRACE,
	";": token.SEMICOLON,
	":": token.COLON,

	"break":    token.BREAK,
	"case":     token.CASE,
	"chan":     token.CHAN,
	"const":    token.CONST,
	"continue": token.CONTINUE,

	"default":     token.DEFAULT,
	"defer":       token.DEFER,
	"else":        token.ELSE,
	"fallthrough": token.FALLTHROUGH,
	"for":         token.FOR,

	"func":   token.FUNC,
	"go":     token.GO,
	"goto":   token.GOTO,
	"if":     token.IF,
	"import": token.IMPORT,

	"interface": token.INTERFACE,
	"map":       token.MAP,
	"package":   token.PACKAGE,
	"range":     token.RANGE,
	"return":    token.RETURN,

	"select": token.SELECT,
	"struct": token.STRUCT,
	"switch": token.SWITCH,
	"type":   token.TYPE,
	"var":    token.VAR,

	"~": token.TILDE,
}
