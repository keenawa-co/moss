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

// -----

type Base struct {
	_        [0]int
	severity Severity
	summary  string
	detail   string
	address  string
}

func NewBaseDiagnostic(severity Severity, summary, detail string) Base {
	return Base{
		severity: severity,
		summary:  summary,
		detail:   detail,
	}
}

func (d Base) Severity() Severity {
	return d.severity
}

func (d Base) Description() Description {
	return Description{
		Summary: d.summary,
		Detail:  d.detail,
		Address: d.address,
	}
}

func (d Base) Source() Source {
	return Source{}
}

// -----

type NativeError struct {
	err error
}

func NewNativeError(err error) NativeError {
	return NativeError{
		err: err,
	}
}

func (e NativeError) Severity() Severity {
	return Error
}

func (e NativeError) Description() Description {
	return Description{
		Summary: e.err.Error(),
	}
}

func (e NativeError) Source() Source {
	return Source{}
}

// -----

var (
	_ Diagnostic = Base{}
	_ Diagnostic = NativeError{}
)
