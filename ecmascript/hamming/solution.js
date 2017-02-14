const split = str => str.split('');
const compare = (a, b) => a === b;

export default class Hamming {
  compute(parent, child) {
    let diff = 0;

    if (parent.length !== child.length) {
      throw new Error('DNA strands must be of equal length.');
    }

    split(parent).forEach((l, i) => {
      if (!compare(l, split(child)[i])) {
        diff += 1;
      }
    });

    return diff;
  }
}
