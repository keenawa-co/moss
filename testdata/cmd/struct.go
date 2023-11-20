package main

import "github.com/4rchr4y/testdata/internal/api"

type FirstStruct struct {
	A, B string
	a, b string
	C    int
	D    float32
}

func (f *FirstStruct) FirstStructMethodA(a, b string) (*api.Body, error) {
	if a != b {
		if len(b) > 5 {
			if len(a) < 5 {
				a = b
			}
		}
	}

	return nil, nil
}

func (f *FirstStruct) FirstStructMethodB(a int, b string, c any) (string, error) {
	if f.B != b {
		return f.B, nil
	}

	return b, nil
}
