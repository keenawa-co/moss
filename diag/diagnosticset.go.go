package diag

import (
	"fmt"

	"github.com/hashicorp/hcl/v2"
)

type DiagnosticSet []Diagnostic

func (ds DiagnosticSet) Length() int                { return len(ds) }
func (ds DiagnosticSet) AsList() []Diagnostic       { return ds }
func (ds DiagnosticSet) ByIndex(idx int) Diagnostic { return ds[idx] }

func (ds DiagnosticSet) Append(items ...interface{}) DiagnosticSet {
	for i := range items {
		if items[i] == nil {
			continue
		}

		switch value := items[i].(type) {
		case Diagnostic:
			ds = append(ds, value)
		case DiagnosticSet:
			ds = append(ds, value...)
		case *hcl.Diagnostic:
			ds = append(ds, hclDiagnostic{value})
		case hcl.Diagnostics:
			for i := range value {
				ds = append(ds, hclDiagnostic{value[i]})
			}
		default:
			panic(fmt.Sprintf("unsupported diagnostic(s) type: %T", value))
		}
	}

	if len(ds) < 1 {
		return nil
	}

	return ds
}

func (ds DiagnosticSet) HasError() bool {
	for i := range ds {
		if ds[i].Severity() == Error {
			return true
		}
	}

	return false
}
