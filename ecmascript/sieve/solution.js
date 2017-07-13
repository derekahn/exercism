export default class Sieve {
  constructor(limit) {
    this.primes = [];
    this.getPrimesUpTo(limit);
  }

  getPrimesUpTo(limit, start = 2) {
    Array(start + limit)
      .fill(0)
      .map( (_, i) => i )
      .slice(start)
      .filter(i => i <= limit)
      .reduce((sieve, i) => {

        if (!sieve[i]) {
          this.primes.push(i);

          for (let j = 2 * i; j <= limit; j += i) {
            sieve[j] = true;
          }

        }
        return sieve;
      }, []);
  }
}
