package sieve

const testVersion = 1

func Sieve(max int) []int {
	in := make(chan int)
	fin := make(chan bool)
	go generate(in, fin, max)
	return process(in, fin)
}

func process(in chan int, fin chan bool) (primes []int) {
	for {
		select {
		case <-fin:
			close(fin)
			return
		default:
			prime := <-in
			primes = append(primes, prime)
			out := make(chan int)
			go filter(in, out, prime)
			in = out
		}
	}
}

func generate(ch chan int, fin chan bool, max int) {
	for i := 2; i <= max; i++ {
		ch <- i
		if i == max {
			fin <- true
		}
	}
	close(ch)
}

func filter(in, out chan int, prime int) {
	for {
		i := <-in
		if i%prime != 0 {
			out <- i
		}
	}
}
