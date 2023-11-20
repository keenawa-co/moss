package main

type SecondStruct[T1 comparable, T2 any] struct {
	A FirstStruct
	B struct {
		A, B string
		C    int
		D    float32
	}
}

func (s *SecondStruct[T1, T2]) SecondStructMethodA(a int, b string, c any) string {
	return s.B.A
}
