export default class Triangle {
  constructor(...sides) {
    this.sides = sides;
  }

  kind() {
    if (this.violatesInequality() || this.hasImpossibleSides()) {
      throw new TypeError("illegal");
    }

    switch (this.uniqueSidesLength()) {
      case 1:
        return "equilateral";
      case 2:
        return "isosceles";
      default:
        return "scalene";
    }
  }

  violatesInequality() {
    const [a, b, c] = this.sides;
    return a + b < c || a + c < b || b + c < a;
  }

  hasImpossibleSides() {
    const [a, b, c] = this.sides;
    return a <= 0 || b <= 0 || c <= 0;
  }

  uniqueSidesLength() {
    return new Set(this.sides).size;
  }
}
