package sieve

const testVersion = 1

func Sieve(limit int) (primes []int) {
	sieve := make([]bool, limit)

	for i := 2; i < limit; i++ {
		if !sieve[i] {
			primes = append(primes, i)
			for j := 2 * i; j < limit; j += i {
				sieve[j] = true
			}
		}
	}

	return
}
