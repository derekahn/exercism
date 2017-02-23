export default class Pangram {
  constructor(str) {
    this.sentence = str;
  }

  isPangram() {
    // Create object with key of letter and default val false
    let alpha = Array(26)
                .fill({})
                .reduce((obj, _, i) => {
                  obj[String.fromCharCode(97 + i)] = false;
                  return obj;
                }, {});

    // Toggle letter
    this
    .sentence
    .split('')
    .forEach(l => {
      if (typeof alpha[l.toLowerCase()] !== 'undefined') {
        alpha[l.toLowerCase()] = true;
      }
    });

    // Test for falsey
    return !Object
      .keys(alpha)
      .map(l => alpha[l])
      .includes(false);
  }
}
