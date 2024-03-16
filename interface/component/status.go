package component

type Status uint32

const (
	StatusUnknown Status = iota << 0 // indicates incorrect implementation
	StatusOK                         // stable, ready to go
)

var StatusToString = [...]string{
	StatusUnknown: "Unknown",
	StatusOK:      "Ok",
}

func (s Status) String() string { return StatusToString[s] }
