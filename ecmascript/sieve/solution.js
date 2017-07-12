export default class Sieve {
  constructor(limit) {
    this.primes = [];
    this.getPrimesUpTo(limit);
  }

  getPrimesUpTo(limit) {
    const sieve = [];

    for (let i = 2; i <= limit; i++) {
      if (!sieve[i]) {
        this.primes.push(i);
        for (let j = 2 * i; j <= limit; j += i) {
          sieve[j] = true
        }
      }
    }
  }
}
