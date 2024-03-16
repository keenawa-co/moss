package diag

type Severity rune

const (
	Error   Severity = 'E'
	Warning Severity = 'W'
)

type Description struct {
	_                        [0]int
	Address, Summary, Detail string
}

type Range struct {
	_          [0]int
	Filename   string
	Start, End Pos
}

type Pos struct {
	_                  [0]int
	Line, Column, Byte int
}

type Source struct {
	_       [0]int
	Subject *Range
	Context *Range
}

type Diagnostic interface {
	Severity() Severity
	Description() Description
	Source() Source
}
