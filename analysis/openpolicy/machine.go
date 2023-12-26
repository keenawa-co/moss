package openpolicy

import (
	"crypto/sha256"
	"encoding/hex"
	"fmt"
	"sort"
	"strings"

	"github.com/4rchr4y/goray/pkg/radix"
	"github.com/open-policy-agent/opa/ast"
)

// Linker
type metaSet struct {
	set map[string]map[string]struct{}
}

func newMetaSet() *metaSet {
	return &metaSet{
		set: make(map[string]map[string]struct{}),
	}
}

func (ms *metaSet) hasVendor(groupHash, vendor string) bool {
	if _, exists := ms.set[groupHash]; !exists {
		return false
	}

	if _, exists := ms.set[groupHash][vendor]; !exists {
		return false
	}

	return true
}

func (ms *metaSet) saveVendor(groupHash, vendor string) {
	if _, exists := ms.set[groupHash]; !exists {
		ms.set[groupHash] = make(map[string]struct{})
	}

	ms.set[groupHash][vendor] = struct{}{}
}

type registry struct {
	idxByPath    *radix.Tree[int]
	idxByPackage *radix.Tree[int]
	store        []*RegoFile
}

func (r *registry) insert(file *RegoFile) {
	r.idxByPath.Store([]byte(file.Path), len(r.store))
	r.idxByPackage.Store([]byte(file.Parsed.Package.Path.String()), len(r.store))
	r.store = append(r.store, file)
}

func (r *registry) load(key string) ([]*RegoFile, bool) {
	if strings.Contains(key, "/") {
		idxList, exists := r.idxByPath.LoadPrefix([]byte(key))
		if !exists {
			return nil, false
		}

		result := make([]*RegoFile, len(idxList))
		for idx := range idxList {
			result[idx] = r.store[idxList[idx].Value]
		}

		return result, true
	}

	if strings.Contains(key, ".") {
		idxList, exists := r.idxByPackage.LoadPrefix([]byte(key))
		if !exists {
			return nil, false
		}
		result := make([]*RegoFile, len(idxList))
		for idx := range idxList {
			result[idx] = r.store[idxList[idx].Value]
		}

		return result, true
	}

	return nil, true
}

func newRegistry() *registry {
	return &registry{
		idxByPath:    radix.NewTree[int](),
		idxByPackage: radix.NewTree[int](),
		store:        make([]*RegoFile, 0),
	}
}

type Compiler interface {
	Compile(map[string]*ast.Module) (Compiler, error)
}

type Machine struct {
	loader   *Loader
	meta     *metaSet
	registry *registry
	cgroup   map[string]Compiler
}

func NewMachine(loader *Loader) *Machine {
	return &Machine{
		loader:   loader,
		meta:     newMetaSet(),
		registry: newRegistry(),
		cgroup:   make(map[string]Compiler),
	}
}

// compFn compileFn, options ...compileOptFn
func (m *Machine) Compile() ([]*ast.Compiler, error) {
	r := make([]*ast.Compiler, 0, len(m.meta.set))

	for _, v := range m.meta.set {
		c, err := m.compileGroup(NewCompiler(), v)
		if err != nil {
			return nil, err
		}

		r = append(r, c.Rc)
	}

	return r, nil
}

func (m *Machine) compileGroup(compiler *compiler, group map[string]struct{}) (*compiler, error) {
	modules := make(map[string]*ast.Module, len(group))

	for k := range group {
		files, _ := m.registry.load(k)

		for _, f := range files {
			modules[k] = f.Parsed
		}

	}

	c, err := compiler.Compile(modules)
	if err != nil {
		return nil, err
	}

	return c, nil
}

func (m *Machine) RegisterPolicy(p *Policy) error {
	tgh := hash(p.Targets)    // creating target group hash
	m.registry.insert(p.File) // file registration
	m.meta.saveVendor(tgh, p.File.Path)

	fmt.Println(p.Vendors)

	for i := range p.Vendors {

		if exists := m.meta.hasVendor(tgh, p.Vendors[i]); exists {
			continue
		}

		if _, exists := m.registry.load(p.Vendors[i]); exists {
			m.meta.saveVendor(tgh, p.Vendors[i])
			continue
		}

		info, err := m.loader.fs.Stat(p.Vendors[i])
		if err != nil {
			return err
		}

		if !info.IsDir() {
			file, err := m.loader.LoadRegoFile(p.Vendors[i])
			if err != nil {
				return err
			}

			m.registry.insert(file)
			m.meta.saveVendor(tgh, p.Vendors[i])

			continue
		}

		files, err := m.loader.LoadDir(p.Vendors[i])
		if err != nil {
			return err
		}

		for f := range files {
			m.registry.insert(files[f])
			m.meta.saveVendor(tgh, files[f].Path)
		}
	}

	for _, i := range p.File.Parsed.Imports {
		files, exists := m.registry.load(i.Path.String())
		if !exists {
			return fmt.Errorf("import %s is undefined", i.Path.String())
		}

		for _, f := range files {
			m.meta.saveVendor(tgh, f.Path)
		}

	}

	return nil
}

func (m *Machine) RegisterBundle(b *Bundle) {
	for i := range b.Files {
		if _, exists := m.registry.load(b.Files[i].Path); exists {
			continue
		}

		m.registry.insert(b.Files[i])
	}
}

func hash(data []string) string {
	sort.Strings(data)
	concatenated := strings.Join(data, "")
	hash := sha256.Sum256([]byte(concatenated))

	return hex.EncodeToString(hash[:])
}

func trimPath(stdDirPath string, libPath string) string {
	return strings.Clone(libPath[len(stdDirPath)+1 : len(libPath)-len(regoFileExt)])
}
