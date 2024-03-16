package diag

var _ Diagnostic = nativeErrorDiagnostic{}

type nativeErrorDiagnostic struct {
	err error
}

func NewNativeErrorDiag(err error) nativeErrorDiagnostic {
	return nativeErrorDiagnostic{
		err: err,
	}
}

func (e nativeErrorDiagnostic) Severity() Severity {
	return Error
}

func (e nativeErrorDiagnostic) Description() Description {
	return Description{
		Summary: e.err.Error(),
	}
}

func (e nativeErrorDiagnostic) Source() Source {
	return Source{}
}
