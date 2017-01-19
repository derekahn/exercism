package triangle

import "math"

const testVersion = 3

type Kind kind
type kind int

const (
	NaT Kind = iota // not a triangle
	Equ Kind = iota // equilateral
	Iso Kind = iota // isosceles
	Sca Kind = iota // scalene
)

func KindFromSides(a, b, c float64) Kind {
	x, y, z := sortSides(a, b, c)

	if math.IsNaN(0*x*y*z) || x <= 0 || (x+y) < z {
		return NaT
	}

	if a == b && b == c {
		return Equ
	}

	if x == y || y == z {
		return Iso
	}

	return Sca
}

func sortSides(a, b, c float64) (x, y, z float64) {

	if a <= b && b <= c {
		return a, b, c
	}

	if a >= b && b >= c {
		return c, b, a
	}

	if b < a {
		if a < c {
			return b, a, c
		}
		return b, c, a
	}

	if a < c {
		return a, c, b
	}

	return c, a, b
}
