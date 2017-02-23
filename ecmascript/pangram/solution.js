export default class Pangram {
  constructor(str) {
    this.sentence = str;
  }

  isPangram() {
    const alphabet = [...Array(26)].map((_, i) => String.fromCharCode(i+97));
    const sentence = this.sentence.toLowerCase();

    return alphabet.every(element => sentence.indexOf(element) !== -1);
  }
}
