import BigInt from "./big-integer";

export default class Grains {
  square(num) {
    return BigInt(2).pow(num - 1).toString();
  }
  total() {
    let total = BigInt(0);

    for (let squareNum = 1; squareNum <= 64; squareNum++) {
      total = total.add(this.square(squareNum));
    }

    return total.toString();
  }
}
