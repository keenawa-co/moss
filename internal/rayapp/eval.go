package rayapp

import (
	"fmt"

	"github.com/4rchr4y/goray/diag"
	"github.com/4rchr4y/goray/internal/config"
	"github.com/4rchr4y/goray/internal/hcllang"
	"github.com/hashicorp/hcl/v2"
	"github.com/zclconf/go-cty/cty"
)

type Evaluator struct {
	Config      *config.Config
	NamedValues *hcllang.NamedValueSet
}

type EvalDataSelector struct {
	Evaluator  *Evaluator
	ModulePath string
}

func (selector *EvalDataSelector) GetLetValue(addr hcllang.LetVariable, rng hcl.Range) (cty.Value, diag.DiagnosticSet) {
	var diags diag.DiagnosticSet

	modConf, ok := selector.Evaluator.Config.Children[selector.ModulePath]
	if !ok {
		panic(fmt.Sprintf("Module %q is undefined", selector.ModulePath))
	}

	if _, ok := modConf.Module.Variables[addr.Name]; !ok {
		diags = diags.Append(&hcl.Diagnostic{
			Severity: hcl.DiagError,
			Summary:  `Reference to undeclared let variable`,
			Detail:   fmt.Sprintf(`Let variable with the name %q has not been declared`, addr.Name),
			// Subject:  rng.ToHCL().Ptr(),
		})

		return cty.DynamicVal, diags
	}

	absVarPath := fmt.Sprintf("%s.%s", selector.ModulePath, addr.Name)
	val := selector.Evaluator.NamedValues.GetLetVariableValue(absVarPath)

	return val, diags
}

func (selector *EvalDataSelector) GetPropertyValue(addr hcllang.PropertyVariable, rng hcl.Range) (cty.Value, diag.DiagnosticSet) {
	var diags diag.DiagnosticSet

	modConf, ok := selector.Evaluator.Config.Children[selector.ModulePath]
	if !ok {
		panic(fmt.Sprintf("Module %q is undefined", selector.ModulePath))
	}

	if _, ok := modConf.Module.Header.Variables[addr.Name]; !ok {
		diags = diags.Append(&hcl.Diagnostic{
			Severity: hcl.DiagError,
			Summary:  `Reference to undeclared property variable`,
			Detail:   fmt.Sprintf(`Property variable with the name %q has not been declared`, addr.Name),
			// Subject:  rng.ToHCL().Ptr(),
		})

		return cty.DynamicVal, diags
	}

	absVarPath := fmt.Sprintf("%s.%s", selector.ModulePath, addr.Name)
	val := selector.Evaluator.NamedValues.GetPropVariableValue(absVarPath)
	fmt.Println("---", selector.ModulePath)
	return val, diags
}
