package openpolicy

import (
	"crypto/sha256"
	"encoding/hex"
	"fmt"
	"sort"
	"strings"

	"github.com/open-policy-agent/opa/ast"
)

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
	indexByPath    map[string]int
	indexByPackage map[string]int
	bucket         []*RegoFile
}

func (r *registry) store(file *RegoFile) {
	r.indexByPath[file.Path] = len(r.bucket)
	r.indexByPackage[file.Parsed.Package.Path.String()] = len(r.bucket)

	r.bucket = append(r.bucket, file)
}

func (r *registry) load(key string) (*RegoFile, bool) {
	if strings.Contains(key, "/") {
		index, exists := r.indexByPath[key]
		if !exists {
			return nil, false
		}

		return r.bucket[index], true
	}

	if strings.Contains(key, ".") {
		index, exists := r.indexByPackage[key]
		if !exists {
			return nil, false
		}

		return r.bucket[index], true
	}

	return nil, false
}

type Compiler interface {
	Compile(map[string]*ast.Module) (Compiler, error)
}

func newRegistry() *registry {
	return &registry{
		indexByPath:    make(map[string]int),
		indexByPackage: make(map[string]int),
		bucket:         make([]*RegoFile, 0),
	}
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
	files := make(map[string]*ast.Module, len(group))

	for k := range group {
		fmt.Println(k)
		file, _ := m.registry.load(k)
		files[k] = file.Parsed
	}

	c, err := compiler.Compile(files)
	if err != nil {
		return nil, err
	}

	return c, nil
}

func (m *Machine) RegisterPolicy(p *Policy) error {
	tgh := hash(p.Targets)   // creating target group hash
	m.registry.store(p.File) // file registration
	m.meta.saveVendor(tgh, p.File.Path)

	for i := range p.Vendors {
		if exists := m.meta.hasVendor(tgh, p.Vendors[i]); exists {
			continue
		}

		if _, exists := m.registry.load(p.Vendors[i]); exists {
			m.meta.saveVendor(tgh, p.Vendors[i])
			continue
		}

		file, err := m.loader.LoadRegoFile(p.Vendors[i])
		if err != nil {
			return err
		}

		m.registry.store(file)
		m.meta.saveVendor(tgh, p.Vendors[i])
	}

	for _, i := range p.File.Parsed.Imports {
		file, exists := m.registry.load(i.Path.String())
		if !exists {
			return fmt.Errorf("import %s is undefined", i.Path.String())
		}

		m.meta.saveVendor(tgh, file.Path)
	}

	return nil
}

func (m *Machine) RegisterBundle(b *Bundle) {
	for i := range b.Files {
		if _, exists := m.registry.load(b.Files[i].Path); exists {
			continue
		}

		m.registry.store(b.Files[i])
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
