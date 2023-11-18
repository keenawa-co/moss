package obj

import (
	"bytes"
	"fmt"
	"go/ast"
	"go/format"
)

// objKindFromString function is necessary for cases when there is *ast.Object
// and it is necessary to convert it to a local Object
func objKindFromString(value string) ObjKind {
	for i, v := range objKindStrings {
		if v == value {
			return ObjKind(i)
		}
	}

	return Bad
}

func determineExprType(fobj *FileObj, expr ast.Expr, adder func(filed *FieldObj)) (any, error) {
	switch e := expr.(type) {
	case *ast.StructType:
		return NewStructTypeObj(fobj, e)

	case *ast.SelectorExpr:
		ident, ok := e.X.(*ast.Ident)
		if !ok {
			return e.Sel.Name, nil
		}

		if _, exists := fobj.Imports.Cache[ident.Name]; exists {
			adder(&FieldObj{
				Names: []*IdentObj{{
					Name: ident.Name,
					Kind: Var,
				}},
				Type: e.Sel.Name,
			})
		}

		return e.Sel.Name, nil

	case *ast.StarExpr:
		// This branch (in my understanding) is used only to determine the receiver of methods.
		// Getting into this branch means that receiver's type is a pointer.
		return determineExprType(fobj, e.X, adder)

	case *ast.IndexListExpr:
		return determineExprType(fobj, e.X, adder)

	case *ast.Ident:
		// This branch (in my understanding) is used only to determine the receiver of methods.
		// Getting into this branch means that receiver's type is not a pointer, it's a regular struct.
		// So return the name of the struct.
		return e.Name, nil

	default:
		var buf bytes.Buffer
		if err := format.Node(&buf, fobj.FileSet, e); err != nil {
			return nil, fmt.Errorf("failed to format expr: %w", err)
		}

		return buf.String(), nil
	}
}

func processField(fobj *FileObj, field *ast.Field, adder func(filed *FieldObj)) (*FieldObj, error) {
	typ, err := determineExprType(fobj, field.Type, adder)
	if err != nil {
		return nil, err
	}

	names := make([]*IdentObj, 0, len(field.Names))
	for _, name := range field.Names {
		names = append(names, &IdentObj{
			Name: name.Name,
			Kind: objKindFromString(name.Obj.Kind.String()),
		})
	}

	return &FieldObj{
		Names: names,
		Type:  typ,
	}, nil
}

func ProcessFieldList(fobj *FileObj, fieldList *ast.FieldList, adder func(filed *FieldObj)) (*FieldObjList, error) {
	fieldObjList := &FieldObjList{
		List: make([]*FieldObj, 0, len(fieldList.List)),
	}

	for _, field := range fieldList.List {
		fieldObj, err := processField(fobj, field, adder)
		if err != nil {
			return nil, err
		}

		fieldObjList.List = append(fieldObjList.List, fieldObj)
	}

	return fieldObjList, nil
}
