package openpolicy

type VendorType int

const (
	TypeRegoFile VendorType = iota
	TypeRegoDir
)

type VendorDescription struct {
	Path string
	Type VendorType
}

func (vd *VendorDescription) IsDir() bool {
	return vd.Type == TypeRegoDir
}

type TargetDescription struct {
	Path string
}

type TargetGroup struct {
	Hash string // hash based on a list of targets
	// List
}

type PolicyDescription struct {
	File    *RegoFile
	Targets []string
	Vendors []*VendorDescription
}

type Policy struct {
	File    *RegoFile
	Targets *TargetGroup
	Vendors []*RegoFile
}
