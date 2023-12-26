package openpolicy

type Policy struct {
	File    *RegoFile
	Targets []string
	Vendors []string
}
