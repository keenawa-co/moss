package smorpher

import (
	"encoding/json"
	"fmt"
	"reflect"
	"strings"

	"github.com/4rchr4y/goray/pkg/maps"
)

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

func (idx *indexTable) debug() string {
	j, _ := json.Marshal(idx.table)
	return string(j)
}

func newIndexTable() *indexTable {
	return &indexTable{
		table: make(map[string]*origin),
	}
}

func sourceMapIndexing(v reflect.Value, path string) *indexTable {
	idxt := newIndexTable()

	for _, key := range v.MapKeys() {
		val := v.MapIndex(key)

		currentPath := buildPath(path, key.String())
		idxt.store(currentPath, &origin{
			name:   key.String(),
			path:   currentPath,
			refval: val,
		})

		embed := sourceValueIndexing(val, currentPath)
		if embed != nil {
			idxt.table = maps.Merge(idxt.table, embed.table)
		}
	}

	return idxt
}

func sourceValueIndexing(val reflect.Value, currentPath string) *indexTable {
	elem := val.Elem()
	switch elem.Kind() {
	case reflect.Map:
		return sourceMapIndexing(elem, currentPath)
	case reflect.Slice:
		return sourceSliceIndexing(elem, currentPath)
	default:
		return nil
	}
}

func sourceSliceIndexing(v reflect.Value, currentPath string) *indexTable {
	idxt := newIndexTable()

	for i := 0; i < v.Len(); i++ {
		sliceElem := v.Index(i)
		if sliceElem.Kind() == reflect.Map {
			indexedPath := buildSliceElemPath(currentPath, i)
			sit := sourceMapIndexing(sliceElem, indexedPath)
			if sit != nil {
				idxt.table = maps.Merge(idxt.table, sit.table)
			}
		}
	}

	return idxt
}

func destStructIndexing(sit idxTable, v reflect.Value, path string, tag string) *indexTable {
	v = deRef(v)
	idxt := newIndexTable()

	for i := 0; i < v.NumField(); i++ {
		field := v.Field(i)
		typeField := v.Type().Field(i)
		currentPath := buildPath(path, getFieldTag(typeField, tag))

		effectiveField := getEffectiveField(field)
		idxt.store(currentPath, &origin{
			name:   typeField.Name,
			path:   currentPath,
			refval: field,
		})

		embed := destFieldIndexing(sit, effectiveField, currentPath, tag)
		if embed != nil {
			idxt.table = maps.Merge(idxt.table, embed.table)
		}
	}

	return idxt
}

func destFieldIndexing(sit idxTable, v reflect.Value, currentPath string, tag string) *indexTable {
	switch deRef(v).Kind() {
	case reflect.Slice:
		return destSliceIndexing(sit, v, currentPath, tag)
	case reflect.Struct:
		return destStructIndexing(sit, v, currentPath, tag)
	default:
		return nil
	}
}

func destSliceIndexing(sit idxTable, field reflect.Value, currentPath string, tag string) *indexTable {
	elemType := field.Type().Elem()
	isStructPtr := elemType.Kind() == reflect.Ptr && elemType.Elem().Kind() == reflect.Struct

	if elemType.Kind() != reflect.Struct && !isStructPtr {
		return nil
	}

	loadedElem, ok := sit.load(currentPath)
	if !ok {
		return nil
	}

	amount := loadedElem.refval.Elem().Len()
	if amount == 0 {
		return nil
	}

	field.Set(reflect.MakeSlice(reflect.SliceOf(elemType), amount, amount))
	idxt := newIndexTable()

	for i := 0; i < amount; i++ {
		elem := initSliceElem(field, i, elemType, isStructPtr)
		field.Index(i).Set(elem)
		indexedPath := buildSliceElemPath(currentPath, i)
		embed := destStructIndexing(sit, field.Index(i), indexedPath, tag)
		if embed != nil {
			idxt.table = maps.Merge(idxt.table, embed.table)
		}
	}

	return idxt
}

func initSliceElem(slice reflect.Value, index int, elemType reflect.Type, isPtr bool) reflect.Value {
	if !isPtr {
		return reflect.New(elemType).Elem()
	}

	return reflect.New(elemType.Elem())
}

func getEffectiveField(field reflect.Value) reflect.Value {
	if field.Kind() == reflect.Ptr && field.IsNil() && field.Type().Elem().Kind() == reflect.Struct {
		if field.CanSet() {
			newStruct := reflect.New(field.Type().Elem())
			field.Set(newStruct)
			return newStruct.Elem()
		}
	}
	return field
}

func getFieldTag(field reflect.StructField, tag string) string {
	fieldTag := strings.Split(field.Tag.Get(tag), ",")[0]
	if fieldTag == "" {
		return field.Name
	}

	return fieldTag
}

func deRef(v reflect.Value) reflect.Value {
	if v.Kind() == reflect.Ptr && !v.IsNil() {
		return v.Elem()
	}

	return v
}

func buildPath(basePath, key string) string {
	if basePath != "" {
		return basePath + "." + key
	}

	return key
}

func buildSliceElemPath(path string, index int) string {
	return fmt.Sprintf("%s.[%d]", path, index)
}
