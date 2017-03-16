export default class Song {
  constructor() {
    this.wriggled = 'wriggled and jiggled and tickled inside her';

    this.rhyme = {
      fly: '',
      spider: `It ${this.wriggled}.\n`,
      bird: `How absurd to swallow a bird!\n`,
      cat: `Imagine that, to swallow a cat!\n`,
      dog: `What a hog, to swallow a dog!\n`,
      goat: `Just opened her throat and swallowed a goat!\n`,
      cow: `I don't know how she swallowed a cow!\n`
    };

    this.animals = [
      'fly',
      'spider',
      'bird',
      'cat',
      'dog',
      'goat',
      'cow',
      'horse'
    ];
  }
  intro(thing) {
    return `I know an old lady who swallowed a ${thing}.\n`;
  }
  hook(loop) {
    const spiderException = animal => animal === 'spider'? `${animal} that ${this.wriggled}` : animal;

    let stanzas = loop < 1 || loop === this.animals.length - 1 ? [''] : new Array(loop).fill('').map((_, i) =>
      `She swallowed the ${this.animals[i+1]} to catch the ${spiderException(this.animals[i])}.\n`
    );

    return stanzas.reverse().join('');
  }
  ending(isEnd) {
    return isEnd === 'horse'?
      `She's dead, of course!\n` :
      `I don't know why she swallowed the fly. Perhaps she'll die.\n`;
  }
  sing() {
    return this.animals.map((animal, i) =>
      [this.intro(animal), this.rhyme[animal], this.hook(i), this.ending(animal)].join('')
    );
  }
  verse(index) {
    return this.sing()[index - 1];
  }
}
