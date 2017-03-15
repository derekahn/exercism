package pythagorean

type Triplet [3]int

func isPythagorean(a, b, c int) bool {
	return a*a+b*b == c*c
}

func Range(min, max int) (triplets []Triplet) {
	for a := min; a <= max; a++ {
		for b := a; b <= max; b++ {
			for c := b; c <= max; c++ {
				if isPythagorean(a, b, c) {
					triplets = append(triplets, Triplet{a, b, c})
				}
			}
		}
	}
	return
}

func Sum(perimeter int) (triplets []Triplet) {
	for _, t := range Range(1, perimeter/2) {
		if (t[0] + t[1] + t[2]) == perimeter {
			triplets = append(triplets, t)
		}
	}
	return
}
