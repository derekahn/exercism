package triangle

import (
	"math"
	"sort"
)

const testVersion = 3

type Kind kind
type kind int

const (
	NaT Kind = iota // not a triangle
	Equ             // equilateral
	Iso             // isosceles
	Sca             // scalene
)

func KindFromSides(a, b, c float64) Kind {
	side := []float64{a, b, c}
	sort.Float64s(side)

	if math.IsNaN(0*a*b*c) || side[0] <= 0 || (side[0]+side[1]) < side[2] {
		return NaT
	}

	if a == b && b == c {
		return Equ
	}

	if side[0] == side[1] || side[1] == side[2] {
		return Iso
	}

	return Sca
}
