package baseschema

const (
	VERSION     = "version"
	COMPONENT   = "component"
	MODULE      = "module"
	NAME        = "name"
	VARIABLE    = "variable"
	EDITION     = "edition"
	IMPORT      = "import"
	PACKAGE     = "package"
	TYPE        = "type"
	DEFAULT     = "default"
	DESCRIPTION = "description"
	NULLABLE    = "nullable"
	SENSITIVE   = "sensitive"
	VALIDATION  = "validation"
)

var ReservedList = map[string]struct{}{
	VERSION:     {},
	COMPONENT:   {},
	MODULE:      {},
	NAME:        {},
	VARIABLE:    {},
	EDITION:     {},
	IMPORT:      {},
	PACKAGE:     {},
	TYPE:        {},
	DEFAULT:     {},
	DESCRIPTION: {},
	NULLABLE:    {},
}

func IsReserved(value string) bool {
	_, ok := ReservedList[value]
	return ok
}
