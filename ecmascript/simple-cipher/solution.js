const ALPHA = 'abcdefghijklmnopqrstuvwxyz';

export default class Cipher {
  constructor(key) {
    this.key = this.validate(key);
  }

  validate( key ) {
    if (typeof key === 'undefined') {
      return this.generateKey();
    }
    if (key === '' || key.match(/[^a-z]/, 'g') || key.length < 1) {
      throw new Error('Bad key');
    }
    return key;
  }

  generateKey() {
    const random = num => Math.floor(Math.random() * num);
    const hash   = () => ALPHA[random(ALPHA.length)];
    return Array.apply(null, Array(100)).map(hash).join('');
  }

  xCode(key, input, sign) {
    const mod = (n, m) => (n % m + m) % m;
    return [...input]
      .reduce((output, letter, ii) => {
        const offset = sign * ALPHA.indexOf(key[mod(ii, key.length)]);
        output += ALPHA[mod(ALPHA.indexOf(letter) + offset, ALPHA.length)];
        return output;
      }, '');
  }

  encode(string) {
    return this.xCode(this.key, string, 1);
  }

  decode(string) {
    return this.xCode(this.key, string, -1);
  }
}
