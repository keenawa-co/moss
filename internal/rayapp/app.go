package rayapp

import (
	"fmt"

	"github.com/4rchr4y/godevkit/v3/must"
	"github.com/4rchr4y/goray/diag"
	"github.com/4rchr4y/goray/interface/component"
	"github.com/4rchr4y/goray/internal/config"
	"github.com/4rchr4y/goray/internal/hcllang"
	"github.com/4rchr4y/goray/internal/proto/convert"
	"github.com/4rchr4y/goray/internal/schematica"
	"github.com/hashicorp/hcl/v2/hcldec"
	"github.com/zclconf/go-cty/cty"
)

type RayApp struct {
	Drivers map[string]component.Interface
	Config  *config.Config
}

type ConfigureParams struct{}

func (a *RayApp) Configure(params *ConfigureParams) diag.DiagnosticSet {
	nv := hcllang.NewNamedValueSet()
	nv.SetLetVariableValue("example", cty.StringVal("test"))
	nv.SetPropVariableValue("example_module.module_var_example", cty.StringVal("test"))

	evalCtx := AppEvalContext{
		Evaluator: &Evaluator{
			Config:      a.Config,
			NamedValues: nv,
		},
	}

	for modName, cfg := range a.Config.Children {
		for componentName, c := range cfg.Module.Components {
			describeSchemaOutput := a.Drivers[componentName].DescribeSchema()
			spec := must.Must(schematica.DecodeBlock(describeSchemaOutput.Schema.Root)) // FIXME: must

			val, _, diags := evalCtx.EvaluateBlock(
				c.Config,
				describeSchemaOutput.Schema.Root,
				&EvalDataSelector{
					Evaluator:  evalCtx.Evaluator,
					ModulePath: modName,
				},
			)

			if diags.HasError() {
				for _, v := range diags {
					fmt.Println(v.Description().Detail)
				}
				panic(diags)
			}

			encoded, err := convert.EncodeValue(val, hcldec.ImpliedType(spec).WithoutOptionalAttributesDeep())
			if err != nil {
				panic(err)
			}

			if out := a.Drivers[componentName].Configure(&component.ConfigureInput{
				MessagePack: encoded,
			}); out.Diagnostics.HasError() {
				for _, d := range out.Diagnostics {
					panic(fmt.Sprintf("summary: %s\ndetails: %s",
						d.Description().Summary,
						d.Description().Detail,
					))
				}
			}
		}
	}

	return nil
}
