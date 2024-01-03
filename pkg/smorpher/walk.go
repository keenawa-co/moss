package smorpher

import (
	"fmt"
	"reflect"
)

type Field struct {
	Parent *Field
	Path   []string
	Value  interface{}
	Kind   reflect.Kind
}

type Handler interface {
	Handle(field *Field)
}

func Walk(h Handler, field *Field) {
	val := reflect.ValueOf(field.Value)

	switch val.Kind() {
	case reflect.Map:
		for _, key := range val.MapKeys() {
			mapVal := val.MapIndex(key)
			kind := mapVal.Kind()
			if kind == reflect.Ptr || kind == reflect.Interface {
				mapVal = mapVal.Elem()
			}

			Walk(h, &Field{
				Parent: field,
				Value:  mapVal.Interface(),
				Path:   append(append([]string(nil), field.Path...), key.String()),
				Kind:   mapVal.Kind(),
			})
		}
	case reflect.Slice, reflect.Array:
		for i := 0; i < val.Len(); i++ {
			sliceVal := val.Index(i)
			kind := sliceVal.Kind()
			if kind == reflect.Ptr || kind == reflect.Interface {
				sliceVal = sliceVal.Elem()
			}

			Walk(h, &Field{
				Parent: field,
				Value:  sliceVal.Interface(),
				Path:   append(append([]string(nil), field.Path...), fmt.Sprintf("[%d]", i)),
				Kind:   sliceVal.Kind(),
			})
		}
	}

	if val.Kind() != reflect.Map && val.Kind() != reflect.Slice && val.Kind() != reflect.Array {
		field.Value = val.Interface()
		field.Kind = val.Kind()
	}

	h.Handle(field)
}
