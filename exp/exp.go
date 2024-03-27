package exp

import (
	"fmt"

	"github.com/4rchr4y/godevkit/v3/must"
	"github.com/4rchr4y/goray/diag"
	"github.com/4rchr4y/goray/interface/component"
	"github.com/4rchr4y/goray/internal/config"
	"github.com/4rchr4y/goray/internal/hclutl"
	"github.com/4rchr4y/goray/internal/kernel"
	"github.com/4rchr4y/goray/internal/proto/convert"
	"github.com/4rchr4y/goray/internal/schematica"
	"github.com/hashicorp/hcl/v2"
	"github.com/hashicorp/hcl/v2/hcldec"
)

type RayApp struct {
	Drivers map[string]component.Interface
	Config  *config.Config
}

func (app *RayApp) Configure(params *ConfParams) diag.DiagnosticSet {
	for moduleName, cfg := range app.Config.Children {
		for componentName, c := range cfg.Module.Components {
			describeSchemaOutput := app.Drivers[componentName].DescribeSchema()
			spec := must.Must(schematica.DecodeBlock(describeSchemaOutput.Schema.Root)) // FIXME: must

			val, diags := params.Scope.EvalBlock(
				&hcl.EvalContext{
					Functions: params.Scope.Functions(),
					Variables: params.Modules[moduleName].Variables(),
				},
				c.Config,
				spec,
			)
			if diags.HasErrors() {
				panic(diags)
			}

			encoded, err := convert.EncodeValue(val, hcldec.ImpliedType(spec).WithoutOptionalAttributesDeep())
			if err != nil {
				panic(err)
			}

			if out := app.Drivers[componentName].Configure(&component.ConfigureInput{
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

type DriverConf struct {
	Client component.Interface
}

type ConfParams struct {
	Scope   *hclutl.Scope
	Modules map[string]*kernel.Module
}
