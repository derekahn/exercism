export default class Anagram {
  constructor(word) {
    this.word = word.toLowerCase();
  }
  isSame(word) {
    return this.word === word.toLowerCase();
  }
  isAnagram(word) {
    return this.normalize(this.word) === this.normalize(word);
  }
  normalize(str) {
    return str.toLowerCase().split('').sort().join();
  }
  matches(words) {
    words = Array.isArray(words)? words : Array.from(arguments);
    return words.filter(word => !this.isSame(word) && this.isAnagram(word));
  }
}
