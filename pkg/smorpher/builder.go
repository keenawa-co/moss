package smorpher

import (
	"fmt"

	"reflect"
	"strings"

	"github.com/4rchr4y/goray/pkg/maps"
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

type indexTable struct {
	table map[string]*origin
}

func (idx *indexTable) store(key string, value *origin) {
	idx.table[key] = value
}

func (idx *indexTable) load(key string) (*origin, bool) {
	value, exists := idx.table[key]
	return value, exists
}

func newIndexTable() *indexTable {
	return &indexTable{
		table: make(map[string]*origin),
	}
}

type idxTable interface {
	store(key string, value *origin)
	load(key string) (*origin, bool)
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
	v := reflect.ValueOf(value).Elem()
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

	b.sit, err = sourceIndexing(reflect.ValueOf(source), "")
	if err != nil {
		return nil, fmt.Errorf("failed to build source cache: %v", err)
	}

	b.dit, err = destIndexing(b.sit, v, "", b.tag)
	if err != nil {
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

func sourceIndexing(v reflect.Value, path string) (*indexTable, error) {
	if v.Kind() != reflect.Map {
		return nil, fmt.Errorf("source value type should be a map, got %s", v.Kind().String())
	}

	sit := &indexTable{
		table: make(map[string]*origin, len(v.MapKeys())),
	}

	for _, key := range v.MapKeys() {
		val := v.MapIndex(key)

		currentPath := buildPath(path, key.String())
		sit.store(currentPath, &origin{
			name:   key.String(),
			path:   currentPath,
			refval: val,
		})

		embed, err := processSourceValue(val, currentPath)
		if err != nil {
			return nil, err
		}
		if embed != nil {
			sit.table = maps.Merge(sit.table, embed.table)
		}
	}

	return sit, nil
}

func processSourceValue(val reflect.Value, currentPath string) (*indexTable, error) {
	if !val.IsValid() {
		return nil, fmt.Errorf("invalid value provided")
	}

	elem := val.Elem()
	switch elem.Kind() {
	case reflect.Map:
		return sourceIndexing(elem, currentPath)

	case reflect.Slice:
		for i := 0; i < elem.Len(); i++ {
			sliceElem := elem.Index(i)
			if sliceElem.Kind() == reflect.Map {
				indexedPath := fmt.Sprintf("%s.[%d]", currentPath, i)
				return sourceIndexing(sliceElem, indexedPath)
			}
		}

		return nil, nil
	}

	return nil, nil
}

func destIndexing(sit idxTable, v reflect.Value, path string, tag string) (*indexTable, error) {
	v = deRefPtr(v)

	if v.Kind() != reflect.Struct {
		return nil, fmt.Errorf("destination value type should be a struct or pointer to struct, got %s", v.Kind().String())
	}

	dit := &indexTable{
		table: make(map[string]*origin, v.NumField()),
	}

	for i := 0; i < v.NumField(); i++ {
		field := v.Field(i)
		typeField := v.Type().Field(i)
		currentPath := buildPath(path, getFieldTag(typeField, tag))

		effectiveField := getEffectiveField(field)
		dit.store(currentPath, &origin{
			name:   typeField.Name,
			path:   currentPath,
			refval: field,
		})

		embed, err := processField(sit, effectiveField, currentPath, tag)
		if err != nil {
			return nil, err
		}
		if embed != nil {
			dit.table = maps.Merge(dit.table, embed.table)
		}
	}

	return dit, nil
}

func processField(sit idxTable, field reflect.Value, currentPath string, tag string) (*indexTable, error) {
	switch field.Kind() {
	case reflect.Slice:
		return processSlice(sit, field, currentPath, tag)
	case reflect.Struct:
		return destIndexing(sit, field, currentPath, tag)
	default:
		return nil, nil
	}
}

func initializeSliceElement(slice reflect.Value, index int, elemType reflect.Type, isPtr bool) reflect.Value {
	if !isPtr {
		return reflect.New(elemType).Elem()
	}

	return reflect.New(elemType.Elem())
}

func processSlice(sit idxTable, field reflect.Value, currentPath string, tag string) (*indexTable, error) {
	elemType := field.Type().Elem()
	isStructPtr := elemType.Kind() == reflect.Ptr && elemType.Elem().Kind() == reflect.Struct

	if elemType.Kind() != reflect.Struct && !isStructPtr {
		return nil, nil
	}

	loadedElem, ok := sit.load(currentPath)
	if !ok {
		return nil, nil
	}

	amount := loadedElem.refval.Elem().Len()
	if amount == 0 {
		return nil, nil
	}

	field.Set(reflect.MakeSlice(reflect.SliceOf(elemType), amount, amount))

	for j := 0; j < loadedElem.refval.Elem().Len(); j++ {
		elem := initializeSliceElement(field, j, elemType, isStructPtr)
		field.Index(j).Set(elem)

		return destIndexing(sit, field.Index(j), fmt.Sprintf("%s.[%d]", currentPath, j), tag)
	}

	return nil, nil
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
	if v.Kind() == reflect.Ptr && !v.IsNil() {
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
