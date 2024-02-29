package hcllang

import (
	"github.com/zclconf/go-cty/cty/function"
	"github.com/zclconf/go-cty/cty/function/stdlib"
)

const (
	LocalNamespace           = "kernel"
	DefaultExternalNamespace = "addon"
	IdentSeparator           = "::"
)

type NameIdent struct {
	namespace string
	addon     string
	name      string
}

func NewNameIdent(plugin, name string) NameIdent {
	return NameIdent{
		namespace: DefaultExternalNamespace,
		addon:     plugin,
		name:      name,
	}
}

func (ident NameIdent) Namespace() string { return ident.namespace }
func (ident NameIdent) Addon() string     { return ident.addon }
func (ident NameIdent) Name() string      { return ident.name }

func (ident NameIdent) String() (name string) {
	name = ident.namespace + IdentSeparator

	if ident.addon != "" {
		name += ident.addon + IdentSeparator
	}

	return name + ident.name
}

var builtinFunctionList = map[NameIdent]function.Function{
	newIdent("abs"):             stdlib.AbsoluteFunc,
	newIdent("ceil"):            stdlib.CeilFunc,
	newIdent("chomp"):           stdlib.ChompFunc,
	newIdent("coalescelist"):    stdlib.CoalesceListFunc,
	newIdent("compact"):         stdlib.CompactFunc,
	newIdent("concat"):          stdlib.ConcatFunc,
	newIdent("contains"):        stdlib.ContainsFunc,
	newIdent("csvdecode"):       stdlib.CSVDecodeFunc,
	newIdent("distinct"):        stdlib.DistinctFunc,
	newIdent("element"):         stdlib.ElementFunc,
	newIdent("chunklist"):       stdlib.ChunklistFunc,
	newIdent("flatten"):         stdlib.FlattenFunc,
	newIdent("floor"):           stdlib.FloorFunc,
	newIdent("format"):          stdlib.FormatFunc,
	newIdent("formatdate"):      stdlib.FormatDateFunc,
	newIdent("formatlist"):      stdlib.FormatListFunc,
	newIdent("indent"):          stdlib.IndentFunc,
	newIdent("join"):            stdlib.JoinFunc,
	newIdent("jsondecode"):      stdlib.JSONDecodeFunc,
	newIdent("jsonencode"):      stdlib.JSONEncodeFunc,
	newIdent("keys"):            stdlib.KeysFunc,
	newIdent("log"):             stdlib.LogFunc,
	newIdent("lower"):           stdlib.LowerFunc,
	newIdent("max"):             stdlib.MaxFunc,
	newIdent("merge"):           stdlib.MergeFunc,
	newIdent("min"):             stdlib.MinFunc,
	newIdent("parseint"):        stdlib.ParseIntFunc,
	newIdent("pow"):             stdlib.PowFunc,
	newIdent("range"):           stdlib.RangeFunc,
	newIdent("regex"):           stdlib.RegexFunc,
	newIdent("regexall"):        stdlib.RegexAllFunc,
	newIdent("reverse"):         stdlib.ReverseListFunc,
	newIdent("setintersection"): stdlib.SetIntersectionFunc,
	newIdent("setproduct"):      stdlib.SetProductFunc,
	newIdent("setsubtract"):     stdlib.SetSubtractFunc,
	newIdent("setunion"):        stdlib.SetUnionFunc,
	newIdent("signum"):          stdlib.SignumFunc,
	newIdent("slice"):           stdlib.SliceFunc,
	newIdent("sort"):            stdlib.SortFunc,
	newIdent("split"):           stdlib.SplitFunc,
	newIdent("strrev"):          stdlib.ReverseFunc,
	newIdent("substr"):          stdlib.SubstrFunc,
	newIdent("timeadd"):         stdlib.TimeAddFunc,
	newIdent("title"):           stdlib.TitleFunc,
	newIdent("trim"):            stdlib.TrimFunc,
	newIdent("trimprefix"):      stdlib.TrimPrefixFunc,
	newIdent("trimspace"):       stdlib.TrimSpaceFunc,
	newIdent("trimsuffix"):      stdlib.TrimSuffixFunc,
	newIdent("upper"):           stdlib.UpperFunc,
	newIdent("values"):          stdlib.ValuesFunc,
	newIdent("zipmap"):          stdlib.ZipmapFunc,

	newIdent("getenv"): GetEnvFn,
}

func newIdent(name string) NameIdent {
	return NameIdent{
		namespace: LocalNamespace,
		name:      name,
	}
}
