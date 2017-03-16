export default class Anagram {
  constructor(word) {
    this.word = word;
    this.letters = word
      .split('')
      .reduce((obj, letter) => {
        letter = letter.toLowerCase();
        obj[letter] = 0;
        return obj;
      },
    {});
  }
  matches(...words) {
    words = Array.isArray(words[0])? words[0] : words;
    return words.map(word => {
      let letters = 0;

      word.toLowerCase().split('').forEach(letter => {
        if (typeof this.letters[letter] !== 'undefined') {
          ++letters;
        }
      });

      if (letters === word.length &&
        this.word.length === word.length &&
        this.word.toLowerCase() !== word.toLowerCase()) {
        return word;
      }
    })
    .filter(word => word);
  }
}
