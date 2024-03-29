package hcllang

import (
	"fmt"
	"strings"

	"github.com/4rchr4y/godevkit/v3/must"
	"github.com/4rchr4y/goray/diag"
	"github.com/4rchr4y/goray/internal/schematica"
	"github.com/hashicorp/hcl/v2"
	"github.com/hashicorp/hcl/v2/hcldec"
	"github.com/zclconf/go-cty/cty"
	"github.com/zclconf/go-cty/cty/function"
)

type Data interface {
	GetLetValue(LetVariable, hcl.Range) (cty.Value, diag.DiagnosticSet)
	GetPropertyValue(PropertyVariable, hcl.Range) (cty.Value, diag.DiagnosticSet)
}

type Scope struct {
	Data      Data
	functions map[string]function.Function
}

func NewScope() *Scope {
	scope := &Scope{
		functions: make(map[string]function.Function, len(BuiltinFunctionList)*2),
	}

	builder := strings.Builder{}

	for ident, fn := range BuiltinFunctionList {
		builder.Reset()

		builder.WriteString(ident.Namespace())
		builder.WriteString(IdentSeparator)
		builder.WriteString(ident.Name())

		scope.functions[ident.Name()] = fn
		scope.functions[builder.String()] = fn
	}

	return scope
}

func (ps *Scope) Functions() map[string]function.Function {
	if ps.functions != nil {
		return ps.functions
	}

	panic("functions have not been defined")
}

func (s *Scope) EvalBlock(body hcl.Body, schema *schematica.Block) (cty.Value, diag.DiagnosticSet) {
	spec := must.Must(schematica.DecodeBlock(schema)) // FIXME: must
	refs, diags := ReferencesInBlock(body, schema)
	ctx, ctxDiags := s.EvalContext(refs)
	diags = diags.Append(ctxDiags)
	if diags.HasError() {
		// We'll stop early if we found problems in the references, because
		// it's likely evaluation will produce redundant copies of the same errors.
		return cty.UnknownVal(schema.Type()), diags
	}

	val, evalDiags := hcldec.Decode(body, spec, ctx)
	diags = diags.Append(evalDiags)

	return val, diags
}

func (s *Scope) EvalContext(refs []*Reference) (*hcl.EvalContext, diag.DiagnosticSet) {
	// return s.evalContext(refs, s.SelfAddr)
	return s.evalContext(refs)
}

func (s *Scope) evalContext(refs []*Reference) (*hcl.EvalContext, diag.DiagnosticSet) {
	if s == nil {
		panic("attempt to construct EvalContext for nil Scope")
	}

	var diags diag.DiagnosticSet
	vals := make(map[string]cty.Value)
	funcs := s.Functions()
	ctx := &hcl.EvalContext{
		Variables: vals,
		Functions: funcs,
	}

	if len(refs) == 0 {
		// Easy path for common case where there are no references at all.
		return ctx, diags
	}

	// First we'll do static validation of the references. This catches things
	// early that might otherwise not get caught due to unknown values being
	// present in the scope during planning.

	// staticDiags := s.Data.StaticValidateReferences(refs, selfAddr, s.SourceAddr)
	// diags = diags.Append(staticDiags)
	// if staticDiags.HasErrors() {
	// 	return ctx, diags
	// }

	// The reference set we are given has not been de-duped, and so there can
	// be redundant requests in it for two reasons:
	//  - The same item is referenced multiple times
	//  - Both an item and that item's container are separately referenced.
	// We will still visit every reference here and ask our data source for
	// it, since that allows us to gather a full set of any errors and
	// warnings, but once we've gathered all the data we'll then skip anything
	// that's redundant in the process of populating our values map.

	// wholeModules := map[string]cty.Value{}
	letVariables := map[string]cty.Value{}
	propVariables := map[string]cty.Value{}
	// localValues := map[string]cty.Value{}
	// outputValues := map[string]cty.Value{}
	// pathAttrs := map[string]cty.Value{}
	// terraformAttrs := map[string]cty.Value{}
	// countAttrs := map[string]cty.Value{}
	// forEachAttrs := map[string]cty.Value{}
	// checkBlocks := map[string]cty.Value{}
	// runBlocks := map[string]cty.Value{}
	// var self cty.Value

	for _, ref := range refs {
		rng := ref.SourceRange

		rawSubj := ref.Subject

		// This type switch must cover all of the "Referenceable" implementations
		// in package addrs, however we are removing the possibility of
		// Instances beforehand.
		// switch addr := rawSubj.(type) {
		// case addrs.ResourceInstance:
		// 	rawSubj = addr.ContainingResource()
		// case addrs.ModuleCallInstance:
		// 	rawSubj = addr.Call
		// case addrs.ModuleCallInstanceOutput:
		// 	rawSubj = addr.Call.Call
		// }

		switch subj := rawSubj.(type) {
		case LetVariable:
			val, valDiags := normalizeRefValue(s.Data.GetLetValue(subj, rng))
			diags = diags.Append(valDiags)
			letVariables[subj.Name] = val
		case PropertyVariable:
			val, valDiags := normalizeRefValue(s.Data.GetPropertyValue(subj, rng))
			diags = diags.Append(valDiags)
			propVariables[subj.Name] = val

		default:
			// Should never happen
			panic(fmt.Errorf("Scope.buildEvalContext cannot handle address type %T", rawSubj))
		}
	}

	// vals["module"] = cty.ObjectVal(wholeModules)
	vals["let"] = cty.ObjectVal(letVariables)
	vals["props"] = cty.ObjectVal(propVariables)
	// vals["local"] = cty.ObjectVal(localValues)
	// vals["path"] = cty.ObjectVal(pathAttrs)
	// vals["terraform"] = cty.ObjectVal(terraformAttrs)
	// vals["count"] = cty.ObjectVal(countAttrs)
	// vals["each"] = cty.ObjectVal(forEachAttrs)

	// Checks, outputs, and run blocks are conditionally included in the
	// available scope, so we'll only write out their values if we actually have
	// something for them.
	// if len(checkBlocks) > 0 {
	// 	vals["check"] = cty.ObjectVal(checkBlocks)
	// }

	// if len(outputValues) > 0 {
	// 	vals["output"] = cty.ObjectVal(outputValues)
	// }

	// if len(runBlocks) > 0 {
	// 	vals["run"] = cty.ObjectVal(runBlocks)
	// }

	// if self != cty.NilVal {
	// 	vals["self"] = self
	// }

	return ctx, diags
}

func normalizeRefValue(val cty.Value, diags diag.DiagnosticSet) (cty.Value, diag.DiagnosticSet) {
	if diags.HasError() {
		// If there are errors then we will force an unknown result so that
		// we can still evaluate and catch type errors but we'll avoid
		// producing redundant re-statements of the same errors we've already
		// dealt with here.
		return cty.UnknownVal(val.Type()), diags
	}
	return val, diags
}
