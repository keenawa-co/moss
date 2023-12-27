package openpolicy

import (
	"strings"

	"github.com/4rchr4y/goray/pkg/bloom"
	"github.com/4rchr4y/goray/pkg/radix"
)

type fileRegistry struct {
	idxByPath    *radix.Tree[*RegoFile]
	idxByPackage *radix.Tree[*RegoFile]
}

type policyRegistry struct {
	filter bloom.BloomFilter
	store  []*Policy
}

type registry struct {
	fileReg   *fileRegistry
	policyReg *policyRegistry
}

func (r *registry) insertPolicy(policy *Policy) error {
	if err := r.policyReg.filter.Put([]byte(policy.File.Path)); err != nil {
		return err
	}

	r.policyReg.store = append(r.policyReg.store, policy)
	return nil
}

func (r *registry) insertRegoFile(files ...*RegoFile) {
	for _, f := range files {
		r.fileReg.idxByPath.Store([]byte(f.Path), f)
		r.fileReg.idxByPackage.Store([]byte(f.Parsed.Package.Path.String()), f)
	}
}

func (r *registry) loadPolicy(policyFilePath string) (*Policy, bool, error) {
	exists, err := r.policyReg.filter.MightContain([]byte(policyFilePath))
	if err != nil {
		return nil, false, err
	}
	if !exists {
		return nil, false, nil
	}

	for _, policy := range r.policyReg.store {
		if policy.File.Path == policyFilePath {
			return policy, true, nil
		}
	}

	return nil, false, nil
}

func (r *registry) loadRegoFileSet(targetPath string) ([]*RegoFile, bool) {
	if strings.Contains(targetPath, "/") {
		files, exists := r.fileReg.idxByPath.LoadPrefix([]byte(targetPath))
		if !exists {
			return nil, false
		}

		result := make([]*RegoFile, len(files))
		for i := range files {
			result[i] = files[i].Value
		}

		return result, true
	}

	if strings.Contains(targetPath, ".") {
		files, exists := r.fileReg.idxByPackage.LoadPrefix([]byte(targetPath))
		if !exists {
			return nil, false
		}

		result := make([]*RegoFile, len(files))
		for i := range files {
			result[i] = files[i].Value
		}

		return result, true
	}

	return nil, true
}

func newRegistry(numberOfPolicies int) *registry {
	size, _ := bloom.CalcFilterParams(uint64(numberOfPolicies), 0.01)

	return &registry{
		fileReg: &fileRegistry{
			idxByPath:    radix.NewTree[*RegoFile](),
			idxByPackage: radix.NewTree[*RegoFile](),
		},

		policyReg: &policyRegistry{
			filter: bloom.NewBloomFilter(size),
			store:  make([]*Policy, numberOfPolicies),
		},
	}
}
