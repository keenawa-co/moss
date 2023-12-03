package ason

import "strconv"

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
	Start Pos `json:"Start"`
	End   Pos `json:"End"`
}
