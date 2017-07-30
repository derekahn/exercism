export default class Squares {
  constructor(max) {
    this.max = max;
  }
  get squareOfSums() {
    const sum = Array(this.max).fill(1).reduce((sum, _, i) => {
      return sum += (i+1);
    }, 0);
    return sum * sum;
  }
  get sumOfSquares() {
    return Array(this.max).fill(1).reduce((sum, _, i) => {
      const x = i + 1;
      return sum += x * x;
    }, 0);
  }
  get difference() {
    return this.squareOfSums - this.sumOfSquares;
  }
}
