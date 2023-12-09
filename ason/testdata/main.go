package main

// // Some doc here
// var scope = "scopeValue"

// /*
// Hello World
// */
// var test2 = "test2Value"

// var (
// 	test3 = "test3Value"
// 	test4 = "test4Value"
// )

// var testArray = [5]int{1, 2, 3, 4, 5}

type serConf int

const (
	CACHE_REF  serConf = iota
	FILE_SCOPE serConf = iota
	PKG_SCOPE  serConf = iota
	IDENT_OBJ  serConf = iota
	LOC        serConf = iota
)
