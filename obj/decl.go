package obj

type DeclObj struct {
	Pos      int
	End      int
	Name     *IdentObj
	Type     any
	TypeKind ObjKind
}

func (o *DeclObj) Kind() ObjKind {
	return o.TypeKind
}

func (o *DeclObj) IsExported() bool {
	return o.Name.IsExported()
}

func (o *DeclObj) IsValid() bool {
	return o.Pos != NoPos && o.End != NoPos
}
