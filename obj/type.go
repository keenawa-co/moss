package obj

type TypeObj struct {
	Pos           int
	End           int
	Name          *IdentObj
	Type          any
	TypeKind      ObjKind
	TypeParams    *FieldObjList
	DependsParams *FieldObjList
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

func (o *TypeObj) ImportAdder(filed *FieldObj) {
	if o.DependsParams == nil {
		o.DependsParams = &FieldObjList{
			List: make([]*FieldObj, 0),
		}
	}

	o.DependsParams.List = append(o.DependsParams.List, filed)
}
