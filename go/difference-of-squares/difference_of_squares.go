package diffsquares

func square(n int) int {
	return n * n
}

func SquareOfSums(n int) int {

	sum := 0
	for i := 1; i <= n; i++ {
		sum += i
	}

	return square(sum)
}

func SumOfSquares(n int) int {

	sum := 0
	for i := 1; i <= n; i++ {
		sum += (i * i)
	}

	return sum
}

func Difference(n int) int {
	return SquareOfSums(n) - SumOfSquares(n)
}
