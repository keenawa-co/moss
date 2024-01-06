package smorpher

import (
	"fmt"

	"reflect"
	"strings"
)

type Mode int

const (
	// Autocomplete is a mode used in the Builder to indicate that if a single value is provided
	// for a field that expects a slice, the single value should be automatically converted
	// into a slice with that single value as its element. This is particularly useful when you
	// want to simplify the setting of fields that are expected to be slices but are common to
	// have a single value, avoiding the need to manually wrap the value in a slice.
	Autocomplete Mode = iota + 1
)

const (
	defaultTag = "json"
)

type origin struct {
	name   string
	path   string        // field path in the struct, e.g. data.embed.some
	refval reflect.Value // original reflect value
}

func (o *origin) isSupportedType(field *Field) bool {
	if field.Kind == reflect.Map {
		return true
	}

	return assignable(o.refval.Type().Elem(), reflect.TypeOf(field.Value))
}

type idxTable interface {
	store(key string, value *origin)
	load(key string) (*origin, bool)
	debug() string
}

type Builder struct {
	value any      // destination struct where data will be written
	tag   string   // e.g. 'json', 'xml', 'toml'
	dit   idxTable // destination index table of paths to structure fields
	sit   idxTable // source index table of paths in source map
	mode  Mode
}

type BuilderOptFn func(*Builder)

func NewBuilder(value any, source map[string]any, options ...BuilderOptFn) (b *Builder, err error) {
	v := deRef(reflect.ValueOf(value))
	b = &Builder{
		value: value,
		tag:   defaultTag,
		dit:   newIndexTable(),
		sit:   newIndexTable(),
		mode:  0,
	}

	for i := range options {
		options[i](b)
	}

	sourceValue := reflect.ValueOf(source)
	if sourceValue.Kind() != reflect.Map {
		return nil, fmt.Errorf("source value type should be a map, got %s", v.Kind().String())
	}

	if v.Kind() != reflect.Struct {
		return nil, fmt.Errorf("destination value type should be a struct or pointer to struct, got %s", v.Kind().String())
	}

	b.sit = sourceMapIndexing(sourceValue, "")
	b.dit = destStructIndexing(b.sit, v, "", b.tag)

	return b, nil
}

func WithTag(tag string) BuilderOptFn {
	return func(b *Builder) {
		b.tag = tag
	}
}

func WithMode(mode Mode) BuilderOptFn {
	return func(b *Builder) {
		b.mode = mode
	}
}

func (b *Builder) Handle(field *Field) {
	path := strings.Join(field.Path, ".")
	origin, ok := b.dit.load(path)
	if !ok {
		return
	}

	if !origin.refval.CanSet() {
		return
	}

	fieldVal := reflect.ValueOf(field.Value)
	switch {
	case assignable(origin.refval.Type(), fieldVal.Type()):
		assign(origin.refval, fieldVal)
		return

	case b.mode == Autocomplete && origin.refval.Kind() == reflect.Slice && origin.isSupportedType(field):
		sliceType := origin.refval.Type()
		slice := reflect.MakeSlice(sliceType, 1, 1)
		assign(slice.Index(0), fieldVal)
		origin.refval.Set(slice)
		return
	}
}

func assignable(destType, valType reflect.Type) bool {
	// checking for pointer compatibility
	if destType.Kind() == reflect.Ptr && destType.Elem().Kind() == valType.Kind() {
		return true
	}

	if valType.Kind() == reflect.Ptr && valType.Elem().Kind() == destType.Kind() {
		return true
	}

	// checking for forward type compatibility
	if destType.Kind() != reflect.Ptr && valType.AssignableTo(destType) {
		return true
	}

	// checking for pointer and value compatibility
	if destType.Kind() == reflect.Ptr && valType.AssignableTo(destType.Elem()) {
		return true
	}

	if destType.Kind() == reflect.Slice && valType.Kind() == reflect.Slice {
		return assignable(destType.Elem(), valType.Elem())
	}

	return false
}

func assign(dest, val reflect.Value) {
	if !dest.CanSet() || !val.IsValid() {
		return
	}

	switch {
	case isDirectlyAssignable(dest, val):
		dest.Set(val)
	case isPointerToNonPointer(dest, val):
		assignPointerToNonPointer(dest, val)
	case isNonPointerToPointer(dest, val):
		assignNonPointerToPointer(dest, val)
	case isSliceToSlice(dest, val):
		assignSliceToSlice(dest, val)
	}
}

func isDirectlyAssignable(dest, val reflect.Value) bool {
	return val.Type().AssignableTo(dest.Type())
}

func isPointerToNonPointer(dest, val reflect.Value) bool {
	return dest.Kind() == reflect.Ptr && val.Kind() != reflect.Ptr
}

func isNonPointerToPointer(dest, val reflect.Value) bool {
	return dest.Kind() != reflect.Ptr && val.Kind() == reflect.Ptr
}

func isSliceToSlice(dest, val reflect.Value) bool {
	return dest.Type().Kind() == reflect.Slice && val.Type().Kind() == reflect.Slice
}

func assignPointerToNonPointer(dest, val reflect.Value) {
	newPtr := reflect.New(dest.Type().Elem())
	newPtr.Elem().Set(val)
	dest.Set(newPtr)
}

func assignNonPointerToPointer(dest, val reflect.Value) {
	dest.Set(val.Elem())
}

func assignSliceToSlice(dest, val reflect.Value) {
	destIsPtrSlice := dest.Type().Elem().Kind() == reflect.Ptr
	valIsPtrSlice := val.Type().Elem().Kind() == reflect.Ptr
	newSlice := reflect.MakeSlice(dest.Type(), val.Len(), val.Cap())

	for i := 0; i < val.Len(); i++ {
		elem := val.Index(i)
		if destIsPtrSlice != valIsPtrSlice {
			adjustSliceElementTypes(destIsPtrSlice, newSlice.Index(i), elem)
			continue
		}

		newSlice.Index(i).Set(elem)
	}

	dest.Set(newSlice)
}

func adjustSliceElementTypes(destIsPtrSlice bool, destElem, srcElem reflect.Value) {
	if destIsPtrSlice {
		newPtr := reflect.New(srcElem.Type())
		newPtr.Elem().Set(srcElem)
		destElem.Set(newPtr)
		return
	}

	if !srcElem.IsNil() {
		destElem.Set(srcElem.Elem())
		return
	}
}
