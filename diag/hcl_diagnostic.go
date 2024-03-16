package diag

import "github.com/hashicorp/hcl/v2"

var _ Diagnostic = hclDiagnostic{}

type hclDiagnostic struct {
	diag *hcl.Diagnostic
}

func (d hclDiagnostic) Severity() Severity {
	switch d.diag.Severity {
	case hcl.DiagWarning:
		return Warning
	default:
		return Error
	}
}

func (d hclDiagnostic) Description() Description {
	return Description{
		Summary: d.diag.Summary,
		Detail:  d.diag.Detail,
	}
}

func (d hclDiagnostic) Source() (source Source) {
	if d.diag.Subject != nil {
		rng := SourceRangeFromHCL(*d.diag.Subject)
		source.Subject = &rng
	}
	if d.diag.Context != nil {
		rng := SourceRangeFromHCL(*d.diag.Context)
		source.Context = &rng
	}
	return source
}

func SourceRangeFromHCL(hclRange hcl.Range) Range {
	return Range{
		Filename: hclRange.Filename,
		Start: Pos{
			Line:   hclRange.Start.Line,
			Column: hclRange.Start.Column,
			Byte:   hclRange.Start.Byte,
		},
		End: Pos{
			Line:   hclRange.End.Line,
			Column: hclRange.End.Column,
			Byte:   hclRange.End.Byte,
		},
	}
}
