package hcllang

import (
	"fmt"

	"github.com/4rchr4y/godevkit/v3/must"
	"github.com/4rchr4y/goray/diag"
	"github.com/4rchr4y/goray/internal/schematica"
	"github.com/hashicorp/hcl/v2"
	"github.com/hashicorp/hcl/v2/hcldec"
)

type ParseRef func(traversal hcl.Traversal) (*Reference, diag.DiagnosticSet)

func ReferencesInBlock(body hcl.Body, schema *schematica.Block) ([]*Reference, diag.DiagnosticSet) {
	if body == nil {
		return nil, nil
	}

	spec := must.Must(schematica.DecodeBlock(schema)) // FIXME: !must

	traversals := hcldec.Variables(body, spec)
	return References(traversals)
}

type Referenceable interface {
	String() string
}

type Reference struct {
	Subject     Referenceable
	SourceRange hcl.Range // TODO: diag.Source
	Remaining   hcl.Traversal
}

func References(traversals []hcl.Traversal) ([]*Reference, diag.DiagnosticSet) {
	if len(traversals) == 0 {
		return nil, nil
	}

	refs := make([]*Reference, 0, len(traversals))
	var diags diag.DiagnosticSet

	for _, traversal := range traversals {
		ref, refDiags := parseRef(traversal)
		diags = diags.Append(refDiags)
		if ref == nil {
			continue
		}
		refs = append(refs, ref)
	}

	return refs, diags
}

type LetVariable struct {
	Name string
}

func (lv LetVariable) String() string {
	return "let" + "." + lv.Name
}

type PropertyVariable struct {
	Name string
}

func (lv PropertyVariable) String() string {
	return "props" + "." + lv.Name
}

func parseRef(traversal hcl.Traversal) (*Reference, diag.DiagnosticSet) {
	var diags diag.DiagnosticSet

	root := traversal.RootName()

	switch root {
	case "let":
		// _ -> rng
		name, _, remain, diags := parseSingleAttrRef(traversal)
		return &Reference{
			Subject: LetVariable{Name: name},
			// SourceRange: tfdiags.SourceRangeFromHCL(rng),
			Remaining: remain,
		}, diags

	case "props":
		// _ -> rng
		name, _, remain, diags := parseSingleAttrRef(traversal)
		return &Reference{
			Subject: PropertyVariable{Name: name},
			// SourceRange: tfdiags.SourceRangeFromHCL(rng),
			Remaining: remain,
		}, diags
	default:
		diags = diags.Append(&hcl.Diagnostic{
			Severity: hcl.DiagError,
			Summary:  "Invalid reference",
			Detail:   `The "data" object does not support this operation.`,
			Subject:  traversal[0].SourceRange().Ptr(),
		})
		return nil, diags
	}

}

func parseSingleAttrRef(traversal hcl.Traversal) (string, hcl.Range, hcl.Traversal, diag.DiagnosticSet) {
	var diags diag.DiagnosticSet

	root := traversal.RootName()
	rootRange := traversal[0].SourceRange()

	if len(traversal) < 2 {
		diags = diags.Append(&hcl.Diagnostic{
			Severity: hcl.DiagError,
			Summary:  "Invalid reference",
			Detail:   fmt.Sprintf("The %q object cannot be accessed directly. Instead, access one of its attributes.", root),
			Subject:  &rootRange,
		})
		return "", hcl.Range{}, nil, diags
	}

	if attrTrav, ok := traversal[1].(hcl.TraverseAttr); ok {
		return attrTrav.Name, hcl.RangeBetween(rootRange, attrTrav.SrcRange), traversal[2:], diags
	}

	diags = diags.Append(&hcl.Diagnostic{
		Severity: hcl.DiagError,
		Summary:  "Invalid reference",
		Detail:   fmt.Sprintf("The %q object does not support this operation.", root),
		Subject:  traversal[1].SourceRange().Ptr(),
	})
	return "", hcl.Range{}, nil, diags
}
