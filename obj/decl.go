package obj

type DeclObj struct {
	Pos  int
	End  int
	Name *IdentObj
	Type any
}

func (o *DeclObj) IsExported() bool {
	return o.Name.IsExported()
}
