package openpolicy

import (
	"fmt"
	"strings"

	"github.com/4rchr4y/goray/config"
	"github.com/open-policy-agent/opa/ast"
)

type Module struct {
	Name         string
	Source       *ast.Module
	Target       []string
	Dependencies map[string]*Module
}

func LoadModule(regoCli *RegoClient, path string) (*Module, error) {
	if module, exists := regoCli.cache[path]; exists {
		return module, nil
	}

	content, err := regoCli.system.ReadFile(path)
	if err != nil {
		return nil, err
	}

	source, err := ast.ParseModule(path, string(content))
	if err != nil {
		return nil, err
	}

	module := &Module{
		Source:       source,
		Dependencies: make(map[string]*Module, len(source.Imports)),
	}

	for i := 0; i < len(module.Source.Imports); i++ {
		importStr := module.Source.Imports[i].Path.String()
		importPath := strings.Clone(importStr[5:])

		if dependency, exists := regoCli.cache[importPath]; exists {
			module.Dependencies[importPath] = dependency
			continue
		}

		if loadFn, exists := regoCli.lazy[importPath]; exists {
			dependency, err := loadFn()
			if err != nil {
				return nil, fmt.Errorf("failed to load dependency %s: %v", importStr, err)
			}

			regoCli.cache[importPath] = dependency
			module.Dependencies[importPath] = dependency
			continue
		}

		return nil, fmt.Errorf("dependency %s is not found", importStr)
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

func NewModuleList(regoCli *RegoClient, policyList []*config.Policy) (*ModuleList, error) {
	moduleList := &ModuleList{List: make([]*Module, len(policyList))}

	for i := 0; i < len(policyList); i++ {
		rawPolicy := policyList[i]

		module, err := LoadModule(regoCli, rawPolicy.Source)
		if err != nil {
			return nil, err
		}

		moduleList.List[i] = module
		moduleList.Length += len(module.Dependencies) + 1
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
