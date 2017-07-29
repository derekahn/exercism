export default class Series {
  constructor(numberString) {
    this.numberString = numberString;
    this.digits = this.getDigits();
  }

  getDigits() {
    return [...this.numberString].map(digit => parseInt(digit, 10));
  }

  slices(sliceSize) {
    if (sliceSize > this.digits.length) {
      throw new Error('Slice size is too big.');
    }

    return this.digits.reduce((result, digit, i) => {
      if (i < this.digits.length - sliceSize + 1) {

        const slices = Array(sliceSize).fill(0).reduce((slice, _, j) => {
          slice.push(this.digits[i + j]);
          return slice;
        }, []);

        result.push(slices);
      }
      return result;
    }, []);
  }
}
