const LETTERS = 'abcdefghijklmnopqrstuvwxyz';
const REVERSED_LETTERS = [...LETTERS].reverse().join('');

export default {
  insertSpacing(s, interval) {
    const pattern = new RegExp('.{1,' + interval + '}', 'g');
    return s.match(pattern).join(' ');
  },

  invert(character) {
    if (character.match(/\d/)) {
      this.push(character);
    } else {
      this.push(LETTERS[REVERSED_LETTERS.indexOf(character)]);
    }
  },

  encode(s) {
    let encoded;
    const characters = [];
    [...s.toLowerCase()].forEach(this.invert, characters);
    encoded = this.insertSpacing(characters.join(''), 5);
    return encoded;
  }
};
