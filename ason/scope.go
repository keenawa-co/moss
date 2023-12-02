package ason

type (
	Pos      interface{ pos() }
	NoPos    int
	Position struct {
		Filename string // filename, if any
		Offset   int    // offset, starting at 0
		Line     int    // line number, starting at 1
		Column   int    // column number, starting at 1 (byte count)
	}
)

func (NoPos) pos()     {}
func (*Position) pos() {}

type Loc struct {
	Start Pos `json:"Start"`
	End   Pos `json:"End"`
}
