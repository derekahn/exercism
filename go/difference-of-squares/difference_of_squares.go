package diffsquares

func square(n int) int {
	return n * n
}

func SquareOfSums(n int) int {

	var sum int
	for i := 1; i <= n; i++ {
		sum += i
	}

	return square(sum)
}

func SumOfSquares(n int) int {
	squares := make([]int, n)

	for i := 1; i <= n; i++ {
		squares[i-1] = square(i)
	}

	var sum int
	for _, s := range squares {
		sum += s
	}

	return sum
}

func Difference(n int) int {
	return SquareOfSums(n) - SumOfSquares(n)
}
