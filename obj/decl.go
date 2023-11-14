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

func (o *DeclObj) IsValid() bool {
	return o.Pos != NoPos && o.End != NoPos
}
