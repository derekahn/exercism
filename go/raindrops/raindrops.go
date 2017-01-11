package raindrops

import "strconv"

const testVersion = 2

func Convert(n int) string {
	var factors = make([]int, 7)
	for i := 1; i <= n; i++ {
		if n%i == 0 {
			factors = append(factors, i)
		}
	}

	var word string
	for _, f := range factors {
		switch f {
		case 3:
			word += "Pling"
		case 5:
			word += "Plang"
		case 7:
			word += "Plong"
		}
	}

	if len(word) < 1 {
		return strconv.Itoa(n)
	}

	return word
}
