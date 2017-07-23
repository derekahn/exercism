class Triplets {
  constructor(conditions) {
    this.min = conditions.minFactor || 1;
    this.max = conditions.maxFactor;
    this.sum = conditions.sum;
  }

  toArray() {
    let triplet;
    const triplets = [];
    for (let a = this.min; a < this.max - 1; a++) {
      for (let b = a + 1; b < this.max; b++) {
        for (let c = b + 1; c <= this.max; c++) {
          triplet = new Triplet(a, b, c);
          if (this.isDesired(triplet)) {
            triplets.push(triplet);
          }
        }
      }
    }
    return triplets;
  }

  isDesired(triplet) {
    const { sum } = this;
    return triplet.isPythagorean() && (!sum || triplet.sum() === sum);
  }
}

export default class Triplet {
  constructor(a, b, c) {
    Object.assign(this, { a, b, c });
  }

  isPythagorean() {
    const { a, b, c } = this;
    return a * a + b * b === c * c;
  }

  sum() {
    const { a, b, c } = this;
    return a + b + c;
  }

  product() {
    const { a, b, c } = this;
    return a * b * c;
  }

  static where(conditions) {
    return new Triplets(conditions).toArray();
  }
}
