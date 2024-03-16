package convert

import (
	"fmt"
	"path"
	"runtime"

	"github.com/4rchr4y/goray/diag"
	"github.com/4rchr4y/goray/internal/proto/protopkg"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

var ToProtoSeverity = map[diag.Severity]protopkg.Diagnostic_Severity{
	diag.Error:   protopkg.Diagnostic_ERR,
	diag.Warning: protopkg.Diagnostic_WARN,
}

func ToProtoDiag(d interface{}, severity ...diag.Severity) protopkg.Diagnostic {
	var s protopkg.Diagnostic_Severity
	if len(severity) > 1 {
		s = ToProtoSeverity[severity[0]]
	} else {
		s = protopkg.Diagnostic_ERR
	}

	switch d := d.(type) {
	case error:
		return protopkg.Diagnostic{
			Severity: s,
			Summary:  d.Error(),
		}
	case string:
		return protopkg.Diagnostic{
			Severity: s,
			Summary:  d,
		}
	default:
		panic(fmt.Sprintf("unsupported diagnostic type: %T", d))
	}
}

func ToProtoDiagSet(diagset diag.DiagnosticSet) []*protopkg.Diagnostic {
	if diagset.Length() < 1 {
		return nil
	}

	result := make([]*protopkg.Diagnostic, diagset.Length())
	for i := range diagset.AsList() {
		result[i] = &protopkg.Diagnostic{
			Severity: ToProtoSeverity[diagset.ByIndex(i).Severity()],
			Summary:  diagset.ByIndex(i).Description().Summary,
			Detail:   diagset.ByIndex(i).Description().Detail,
		}
	}

	return result
}

func FromProtoDiagSet(diags []*protopkg.Diagnostic) diag.DiagnosticSet {
	if len(diags) < 1 {
		return nil
	}

	result := make(diag.DiagnosticSet, 0)

	// TODO:

	return result
}

func FromGRPCError(err error) diag.Diagnostic {
	if err == nil {
		return nil
	}

	pc, _, _, ok := runtime.Caller(1)
	if !ok {
		return diag.NewNativeError(err)
	}

	f := runtime.FuncForPC(pc)
	_, requestName := path.Split(f.Name())

	switch status.Code(err) {
	case codes.Unavailable:
		return diag.NewBaseDiagnostic(
			diag.Error,
			"gRPC plugin didn't respond",
			fmt.Sprintf("The plugin ran into an issue and was unable to process the `%s` invocation", requestName),
		)
	case codes.Canceled:
		return diag.NewBaseDiagnostic(
			diag.Error,
			"gRPC request was cancelled",
			fmt.Sprintf("The `%s` request was cancelled", requestName),
		)
	case codes.Unimplemented:
		return diag.NewBaseDiagnostic(
			diag.Error,
			"undefined gRPC plugin method",
			fmt.Sprintf("The `%s` method is undefined for this gRPC plugin", requestName),
		)
	default:
		return diag.NewBaseDiagnostic(
			diag.Error,
			"gRPC plugin error",
			fmt.Sprintf("unexpected error was returned from `%s`: %v", requestName, err),
		)
	}
}
