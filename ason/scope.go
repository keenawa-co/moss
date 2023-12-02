package ason

type Pos interface{}

type Position struct {
	Filename string // filename, if any
	Offset   int    // offset, starting at 0
	Line     int    // line number, starting at 1
	Column   int    // column number, starting at 1 (byte count)
}

type Loc struct {
	Start *Position `json:"Start,omitempty"`
	End   *Position `json:"End,omitempty"`
}
