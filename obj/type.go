package obj

import (
	"go/ast"
	"go/token"
)

type EmbeddedObject interface{}

type TypeObj struct {
	Start token.Pos
	End   token.Pos
	Name  string
	Obj   EmbeddedObject
	Gens  []*StructFieldObj         // generic type params
	Deps  map[string]*DependencyObj // typed dependencies
}

func (o *TypeObj) EmbedObject(obj EmbeddedObject) {
	o.Obj = obj
}

func (o *TypeObj) Type() string {
	return "type"
}

func NewTypeObj(fobj *FileObj, ts *ast.TypeSpec) (*TypeObj, error) {
	var generics []*StructFieldObj
	var deps map[string]*DependencyObj

	if ts.TypeParams != nil {
		generics = make([]*StructFieldObj, 0, len(ts.TypeParams.List))

		extractedFieldsData, err := extractFieldMap(fobj.FileSet, ts.TypeParams.List)
		if err != nil {
			return nil, err
		}

		// saving all collected generics
		generics = append(generics, extractedFieldsData.fieldsSet...)
		deps = make(map[string]*DependencyObj, len(extractedFieldsData.usedPackages))

		for _, d := range extractedFieldsData.usedPackages {
			// if the alias of this dependency is marked as an internal dependency
			importIndex, importExists := fobj.IsInternalDependency(d.Alias)
			if !importExists {
				continue
			}

			// searching for dependencies in already saved dependencies
			dep, depExists := deps[d.Alias]
			if !depExists {
				deps[d.Element] = &DependencyObj{
					ImportIndex: importIndex,
					Usage:       1,
				}

				continue
			}

			// already exists, simply increase the usage counter
			dep.Usage++
		}
	}

	return &TypeObj{
		Start: ts.Pos(),
		End:   ts.End(),
		Name:  ts.Name.Name,
		Gens:  generics,
		Deps:  deps,
	}, nil
}
