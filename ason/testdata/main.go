package main

import "fmt"

// Some doc here
var scope = "scopeValue"

/*
Hello World
*/
var test2 = "test2Value"

var (
	test3 = "test3Value"
	test4 = "test4Value"
)

var testArray = [5]int{1, 2, 3, 4, 5}

func main() {
	cs := ast.ClassSpec{
		Name:       "Hello",
		Visibility: types.Private,
	}

	ts := cs.Tokenify()

	fmt.Println(cs.Plantify(ts))
}
