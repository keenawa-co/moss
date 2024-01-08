package openpolicy

// TODO
// Use a concept os STORAGE and split compiling process and linking

// Linker
// type metaSet struct {
// 	set map[string]map[string]struct{}
// }

// func newMetaSet() *metaSet {
// 	return &metaSet{
// 		set: make(map[string]map[string]struct{}),
// 	}
// }

// func (ms *metaSet) hasVendor(groupHash, vendor string) bool {
// 	if _, exists := ms.set[groupHash]; !exists {
// 		return false
// 	}

// 	if _, exists := ms.set[groupHash][vendor]; !exists {
// 		return false
// 	}

// 	return true
// }

// func (ms *metaSet) saveVendor(groupHash, vendor string) {
// 	if _, exists := ms.set[groupHash]; !exists {
// 		ms.set[groupHash] = make(map[string]struct{})
// 	}

// 	ms.set[groupHash][vendor] = struct{}{}
// }

// type Compiler interface {
// 	Compile(map[string]*ast.Module) (Compiler, error)
// }

// type Machine struct {
// 	loader *Loader
// 	// meta   *metaSet
// 	groups   map[string][]*Policy
// 	registry *registry
// }

// func NewMachine(loader *Loader, numberOfPolicies int) *Machine {
// 	return &Machine{
// 		loader: loader,
// 		// meta:     newMetaSet(),
// 		registry: newRegistry(numberOfPolicies),
// 		groups:   make(map[string][]*Policy),
// 	}
// }

// // compFn compileFn, options ...compileOptFn
// func (m *Machine) Compile() ([]*ast.Compiler, error) {
// 	r := make([]*ast.Compiler, 0, len(m.groups))

// 	for _, v := range m.groups {
// 		c, err := m.compileGroup(NewCompiler(), v)
// 		if err != nil {
// 			return nil, err
// 		}

// 		r = append(r, c.Rc)
// 	}

// 	return r, nil
// }

// func (m *Machine) compileGroup(compiler *compiler, group []*Policy) (*compiler, error) {
// 	modules := make(map[string]*ast.Module, len(group))

// 	for i := range group {
// 		modules[group[i].File.Path] = group[i].File.Parsed
// 		// files, _ := m.registry.loadRegoFileSet(k)

// 		for _, v := range group[i].Vendors {
// 			modules[v.Path] = v.Parsed
// 		}

// 	}

// 	c, err := compiler.Compile(modules)
// 	if err != nil {
// 		return nil, err
// 	}

// 	return c, nil
// }

// func (m *Machine) RegisterPolicy(pd *PolicyDescription) error {
// 	policy, exists, err := m.registry.loadPolicy(pd.File.Path)
// 	if err != nil {
// 		return err
// 	}
// 	if exists {
// 		// TODO
// 		return nil
// 	}

// 	policy = &Policy{
// 		File:    pd.File,
// 		Vendors: make([]*RegoFile, 0),
// 		Targets: &TargetGroup{
// 			Hash: sortAndHash(pd.Targets),
// 		},
// 	}

// 	for i := range pd.Vendors {
// 		files, exists := m.registry.loadRegoFileSet(pd.Vendors[i].Path)
// 		if exists {
// 			policy.Vendors = append(policy.Vendors, files...)
// 			continue
// 		}

// 		// if there is nothing in the registry, then the dependency
// 		// needs to be loaded

// 		if !pd.Vendors[i].IsDir() {
// 			file, err := m.loader.LoadRegoFile(pd.Vendors[i].Path)
// 			if err != nil {
// 				return err
// 			}

// 			policy.Vendors = append(policy.Vendors, file)
// 			m.registry.insertRegoFile(file)
// 			continue
// 		}

// 		files, err := m.loader.LoadDir(pd.Vendors[i].Path)
// 		if err != nil {
// 			return err
// 		}

// 		policy.Vendors = append(policy.Vendors, files...)
// 		m.registry.insertRegoFile(files...)
// 	}

// 	for _, i := range pd.File.Parsed.Imports {
// 		files, exists := m.registry.loadRegoFileSet(i.Path.String())
// 		if !exists {
// 			return fmt.Errorf("import %s is undefined", i.Path.String())
// 		}

// 		policy.Vendors = append(policy.Vendors, files...)
// 	}

// 	m.groups[policy.Targets.Hash] = append(m.groups[policy.Targets.Hash], policy)

// 	return nil
// }

// func (m *Machine) RegisterBundle(b *Bundle) {
// 	for i := range b.Files {
// 		if _, exists := m.registry.loadRegoFileSet(b.Files[i].Path); exists {
// 			continue
// 		}

// 		m.registry.insertRegoFile(b.Files[i])
// 	}
// }

// func sortAndHash(data []string) string {
// 	sort.Strings(data)
// 	concatenated := strings.Join(data, "")
// 	hash := sha256.Sum256([]byte(concatenated))

// 	return hex.EncodeToString(hash[:])
// }

// func trimPath(stdDirPath string, libPath string) string {
// 	// value 5 is a length of ".rego" string
// 	return strings.Clone(libPath[len(stdDirPath)+1 : len(libPath)-5])
// }
