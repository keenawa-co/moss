package diag

var _ Diagnostic = baseDiagnostic{}

type baseDiagnostic struct {
	_        [0]int
	severity Severity
	summary  string
	detail   string
	address  string
}

func NewBaseDiag(severity Severity, summary, detail string) baseDiagnostic {
	return baseDiagnostic{
		severity: severity,
		summary:  summary,
		detail:   detail,
	}
}

func (d baseDiagnostic) Severity() Severity {
	return d.severity
}

func (d baseDiagnostic) Description() Description {
	return Description{
		Summary: d.summary,
		Detail:  d.detail,
		Address: d.address,
	}
}

func (d baseDiagnostic) Source() Source {
	return Source{}
}
