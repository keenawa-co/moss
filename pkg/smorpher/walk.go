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
			Walk(h, &Field{
				Parent: field,
				Value:  val.MapIndex(key).Interface(),
				Path:   append(append([]string(nil), field.Path...), key.String()),
				Kind:   val.MapIndex(key).Elem().Kind(),
			})
		}

	case reflect.Slice, reflect.Array:
		for i := 0; i < val.Len(); i++ {
			Walk(h, &Field{
				Parent: field,
				Value:  val.Index(i).Interface(),
				Path:   append(append([]string(nil), field.Path...), fmt.Sprintf("[%d]", i)),
				Kind:   val.Index(i).Kind(),
			})

		}
	}

	if val.Kind() != reflect.Map && val.Kind() != reflect.Slice && val.Kind() != reflect.Array {
		field.Value = val.Interface()
		field.Kind = val.Kind()
	}

	h.Handle(field)
}
