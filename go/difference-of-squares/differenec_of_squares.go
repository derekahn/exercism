package diffsquares

func SquareOfSums(n int) int {

	var sum int
	for i := 1; i <= n; i++ {
		sum += i
	}

	return sum * sum
}

func SumOfSquares(n int) int {
	squares := make([]int, n)

	for i := 1; i <= n; i++ {
		squares[i-1] = i * i
	}

	var sum int
	for _, s := range squares {
		sum += s
	}

	return sum
}

func Difference(n int) int {
	sqOfSums := SquareOfSums(n)
	sumOfSq := SumOfSquares(n)

	return sqOfSums - sumOfSq
}
