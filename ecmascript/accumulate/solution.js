export default (arr, accumulator) =>
  arr.reduce((prev, next) => [...prev, accumulator(next)], []);
