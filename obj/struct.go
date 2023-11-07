package obj

import (
	"errors"
	"fmt"
	"go/ast"
	"go/token"
)

type (
	StructObjMeta struct {
		LineCount int
	}

	// TODO: get rid of this structure
	StructFieldObj struct {
		Name       string
		Type       string
		Embedded   *StructTypeObj
		Visibility bool
	}

	StructTypeObj struct {
		Name       *string
		Fields     []*StructFieldObj
		Deps       map[string]*DependencyObj
		Incomplete bool
		Valid      bool
		Metadata   *StructObjMeta
	}
)

func (o *StructTypeObj) Type() string {
	return "struct"
}

func (o *StructTypeObj) AddDependency(importIndex int, element string) {
	if _, exists := o.Deps[element]; !exists {
		o.Deps[element] = &DependencyObj{
			ImportIndex: importIndex,
			Usage:       1,
		}

		return
	}

	o.Deps[element].Usage++
}

func NewStructObj(fset *token.FileSet, node ast.Node, name *string) (*StructTypeObj, []UsedPackage, error) {
	structType, err := extractStructType(node)
	if err != nil {
		return nil, nil, fmt.Errorf("node is not a TypeSpec or StructType: %w", err)
	}

	extractedFieldsData, err := extractFieldMap(fset, structType.Fields.List)
	if err != nil {
		return nil, nil, fmt.Errorf("failed to extract struct field map: %w", err)
	}

	structObj := &StructTypeObj{
		Name:       name,
		Fields:     extractedFieldsData.fieldsSet,
		Deps:       make(map[string]*DependencyObj, len(extractedFieldsData.usedPackages)),
		Incomplete: structType.Incomplete,
		Valid:      structType.Struct.IsValid(),
		Metadata: &StructObjMeta{
			LineCount: CalcEntityLOC(fset, structType),
		},
	}

	return structObj, extractedFieldsData.usedPackages, nil
}

// Attempts to extract *ast.StructType from the given AST node.
func extractStructType(node ast.Node) (*ast.StructType, error) {
	switch n := node.(type) {
	case *ast.TypeSpec:
		if structType, ok := n.Type.(*ast.StructType); ok {
			return structType, nil
		}
	case *ast.StructType:
		return n, nil
	}
	return nil, errors.New("node does not contain a StructType")
}

type UsedPackage struct {
	_              [0]int
	Alias, Element string
}

func extractFieldMap(fset *token.FileSet, fieldList []*ast.Field) (*extractedFieldsData, error) {
	fields := make([]*StructFieldObj, 0, len(fieldList))
	usedPackages := make([]UsedPackage, 0, len(fieldList))

	for _, field := range fieldList {
		fieldMetaData, err := ExtractExprAsType(fset, field.Type)
		if err != nil {
			return nil, err
		}

		for i := 0; i < len(fieldMetaData.UsedPackages); i++ {
			usedPackages = append(usedPackages, fieldMetaData.UsedPackages[i])
		}

		if len(field.Names) == 0 {
			fields = append(fields, &StructFieldObj{
				Type:       fieldMetaData.Type,
				Embedded:   fieldMetaData.EmbeddedStruct,
				Visibility: false,
			})

			continue
		}

		for _, name := range field.Names {
			fields = append(fields, &StructFieldObj{
				Name:       name.Name,
				Type:       fieldMetaData.Type,
				Embedded:   fieldMetaData.EmbeddedStruct,
				Visibility: name.IsExported(),
			})
		}
	}

	return &extractedFieldsData{
		fieldsSet:    fields,
		usedPackages: usedPackages,
	}, nil
}
