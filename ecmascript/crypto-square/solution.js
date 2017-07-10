export default class Square {
  constructor(input) {
    this.input = input;
  }

  normalizePlaintext() {
    return this.input.toLowerCase().replace(/[^a-zA-Z0-9]/g,'');
  }

  size() {
    const realLength = Math.sqrt(this.normalizePlaintext().length);
    return Math.ceil(realLength);
  }

  splitRegex() {
    return new RegExp(`.{1,${this.size()}}`, 'g');
  }

  plaintextSegments() {
    return this.normalizePlaintext().match(this.splitRegex());
  }

  ciphertext() {
    return this.plaintextSegments()
      .reduce((column, currentSegment) => {
        currentSegment.split('').forEach((letter, i) =>
          Array.isArray(column[i])? column[i].push(letter) : column.push([letter])
        );
        return column;
      }, [])
      .map(column => column.join(''))
      .join('');
  }

  normalizeCiphertext() {
    return this.ciphertext().match(this.splitRegex()).join(' ');
  }
}
