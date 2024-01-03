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

type Builder struct {
	value       any
	tag         string             // e.g. 'json', 'xml', 'toml'
	destCache   map[string]*origin // cache of paths to structure fields
	sourceCache map[string]*origin // cache of paths in source map
	mode        Mode
}

func (b *Builder) updateDestCache(path, fieldName string, field reflect.Value) {
	b.destCache[path] = &origin{
		name:   fieldName,
		path:   path,
		refval: field,
	}
}

type BuilderOptFn func(*Builder)

func NewBuilder(value any, source map[string]any, options ...BuilderOptFn) (*Builder, error) {
	v := reflect.ValueOf(value).Elem()
	b := &Builder{
		value:       value,
		tag:         "json",
		destCache:   make(map[string]*origin),
		sourceCache: make(map[string]*origin),
		mode:        0,
	}

	for i := range options {
		options[i](b)
	}

	if err := b.buildSourceCache(reflect.ValueOf(source), ""); err != nil {
		return nil, fmt.Errorf("failed to build source cache: %v", err)
	}

	if err := b.buildDestCache(v, ""); err != nil {
		return nil, fmt.Errorf("failed to build destination cache: %v", err)
	}

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

func (b *Builder) buildSourceCache(v reflect.Value, path string) error {
	if v.Kind() != reflect.Map {
		return fmt.Errorf("source value type should be a map, got %s", v.Kind().String())
	}

	for _, key := range v.MapKeys() {
		val := v.MapIndex(key)

		currentPath := buildPath(path, key.String())
		b.processSourceValue(val, currentPath)

		b.sourceCache[currentPath] = &origin{
			name:   key.String(),
			path:   currentPath,
			refval: val,
		}
	}

	return nil
}

func (b *Builder) processSourceValue(val reflect.Value, currentPath string) {
	if val.Elem().Kind() == reflect.Map {
		b.buildSourceCache(val.Elem(), currentPath)
		return
	}

	if val.Elem().Kind() == reflect.Slice {
		for i := 0; i < val.Elem().Len(); i++ {
			elem := val.Elem().Index(i)

			if elem.Kind() == reflect.Map {
				b.buildSourceCache(elem, fmt.Sprintf("%s.[%d]", currentPath, i))
			}
		}
	}
}

func (b *Builder) buildDestCache(v reflect.Value, path string) error {
	v = deRefPtr(v)

	if v.Kind() != reflect.Struct {
		return fmt.Errorf("destination value type should be a struct or pointer to struct, got %s", v.Kind().String())
	}

	for i := 0; i < v.NumField(); i++ {
		field := v.Field(i)
		typeField := v.Type().Field(i)
		currentPath := buildPath(path, getFieldTag(typeField, b.tag))

		effectiveField := getEffectiveField(field)
		b.updateDestCache(currentPath, typeField.Name, effectiveField)
		b.processField(effectiveField, currentPath)
	}

	return nil
}

func (b *Builder) processField(field reflect.Value, currentPath string) {
	switch field.Kind() {
	case reflect.Slice:
		b.processSlice(field, currentPath)
	case reflect.Struct:
		b.buildDestCache(field, currentPath)
	}
}

func (b *Builder) processSlice(field reflect.Value, currentPath string) {
	elemType := field.Type().Elem()
	isStructPtr := elemType.Kind() == reflect.Ptr && elemType.Elem().Kind() == reflect.Struct

	if elemType.Kind() == reflect.Struct || isStructPtr {
		sce := b.sourceCache[currentPath]

		if sce == nil {
			return
		}

		for j := 0; j < sce.refval.Elem().Len(); j++ {
			if j == 0 {
				amount := sce.refval.Elem().Len()
				field.Set(reflect.MakeSlice(reflect.SliceOf(elemType), amount, amount))
			}

			// working with pointers to structures in a slice
			var elem reflect.Value
			if isStructPtr {
				elem = reflect.New(elemType.Elem())
				field.Index(j).Set(elem)
			} else {
				elem = reflect.New(elemType).Elem()
				field.Index(j).Set(elem)
			}

			b.buildDestCache(field.Index(j), fmt.Sprintf("%s.[%d]", currentPath, j))
		}
	}
}

func (b *Builder) Handle(field *Field) {
	path := strings.Join(field.Path, ".")
	origin, exists := b.destCache[path]
	if !exists {
		return
	}

	if !origin.refval.CanSet() {
		return
	}

	fieldVal := reflect.ValueOf(field.Value)

	switch {
	case assignable(origin.refval.Type(), fieldVal.Type()):
		assign(origin.refval, fieldVal)

	case b.mode == Autocomplete && origin.refval.Kind() == reflect.Slice && origin.isSupportedType(field):
		sliceType := origin.refval.Type()
		slice := reflect.MakeSlice(sliceType, 1, 1)

		assign(slice.Index(0), fieldVal)

		origin.refval.Set(slice)
		return
	}
}

func buildPath(basePath, key string) string {
	if basePath != "" {
		return basePath + "." + key
	}

	return key
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

func deRefPtr(v reflect.Value) reflect.Value {
	if v.Kind() == reflect.Ptr {
		return v.Elem()
	}

	return v
}

func getFieldTag(field reflect.StructField, tag string) string {
	fieldTag := strings.Split(field.Tag.Get(tag), ",")[0]
	if fieldTag == "" {
		return field.Name
	}

	return fieldTag
}

func getEffectiveField(field reflect.Value) reflect.Value {
	if field.Kind() == reflect.Ptr && field.Type().Elem().Kind() == reflect.Struct {
		if field.IsNil() {
			// set the pointer of a new structure
			field.Set(reflect.New(field.Type().Elem()))
		}

		return field.Elem()
	}

	return field
}
