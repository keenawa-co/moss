package openpolicy

import (
	"github.com/4rchr4y/goray/config"
	"github.com/open-policy-agent/opa/ast"
	// "github.com/open-policy-agent/opa/loader"
)

type Module struct {
	Name         string
	Source       *ast.Module
	Target       []string
	Dependencies map[string]*Module
}

func LoadModule(regoCli *RegoClient, policyDef *config.PolicyDef) (*Module, error) {
	content, err := regoCli.system.ReadFile(policyDef.Path)
	if err != nil {
		return nil, err
	}

	source, err := ast.ParseModule(policyDef.Path, string(content))
	if err != nil {
		return nil, err
	}

	module := &Module{
		Source:       source,
		Dependencies: make(map[string]*Module, len(source.Imports)),
	}

	return module, nil
}

type ModuleList struct {
	List   []*Module
	Length int
}

func (moduleList *ModuleList) Len() int {
	return moduleList.Length
}

func NewModuleList(regoCli *RegoClient, policyDefList []*config.PolicyDef) (*ModuleList, error) {
	moduleList := &ModuleList{List: make([]*Module, len(policyDefList))}

	for i := range policyDefList {
		module, err := LoadModule(regoCli, policyDefList[i])
		if err != nil {
			return nil, err
		}

		moduleList.List[i] = module
	}

	return moduleList, nil
}

func MergeList(moduleList *ModuleList) map[string]*ast.Module {
	merged := make(map[string]*ast.Module, moduleList.Len())

	for i := 0; i < len(moduleList.List); i++ {
		merged = MergeModule(merged, moduleList.List[i])
	}

	return merged
}

func MergeModule(merged map[string]*ast.Module, module *Module) map[string]*ast.Module {
	merged[module.Name] = module.Source

	for k, v := range module.Dependencies {
		merged[k] = v.Source
	}

	return merged
}
