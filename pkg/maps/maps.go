package maps

func Merge[M ~map[K]V, K comparable, V any](maps ...M) M {
	fullCap := 0
	for _, m := range maps {
		fullCap += len(m)
	}

	merged := make(M, fullCap)
	for _, m := range maps {
		for key, val := range m {
			merged[key] = val
		}
	}

	return merged
}

func MergeFunc[M ~map[K]V, K comparable, V any](conflictFunc func(V, V) V, maps ...M) M {
	fullCap := 0
	for _, m := range maps {
		fullCap += len(m)
	}

	merged := make(M, fullCap)
	for _, m := range maps {
		for key, val := range m {
			if v, ok := merged[key]; ok {
				merged[key] = conflictFunc(v, val)
				continue
			}
			merged[key] = val
		}
	}

	return merged
}
