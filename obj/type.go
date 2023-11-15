package obj

type TypeObj struct {
	Pos          int
	End          int
	Name         *IdentObj
	Type         any
	TypeKind     ObjKind
	TypeParams   *FieldObjList
	Dependencies map[string]int
}

func (o *TypeObj) Kind() ObjKind {
	return o.TypeKind
}

func (o *TypeObj) IsExported() bool {
	return o.Name.IsExported()
}

func (o *TypeObj) IsValid() bool {
	return o.Pos != NoPos && o.End != NoPos
}

func (o *TypeObj) ImportAdder(importIndex int, element string) {
	if o.Dependencies == nil {
		o.Dependencies = make(map[string]int)
	}

	o.Dependencies[element] = importIndex
}
