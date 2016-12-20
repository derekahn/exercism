package hamming

import "errors"

const testVersion = 5

func Distance(a, b string) (int, error) {
	count := 0

	if len(a) != len(b) {
		return -1, errors.New("Out of range")
	}

	for i := range a {
		if a[i] != b[i] {
			count++
		}
	}

	return count, nil
}
