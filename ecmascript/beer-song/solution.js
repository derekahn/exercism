export default class BeerSong {
  static verse(number) {
    let line1 = bottles(number) + ' of beer on the wall, ';
    let line2 = bottles(number).toLowerCase() + ' of beer.\n';
    let line3 = action(number);
    let line4 = nextBottle(number);

    return [line1, line2, line3, line4].join('');
  }

  static sing(first = 99, last = 0) {
    let verses = [];
    for (let i = first; i >= last; i--) {
      verses.push(this.verse(i));
    }

    return verses.join('\n');
  }
}

function bottles(number) {
  switch (number) {
    case 0:
      return 'No more bottles';
    case 1:
      return '1 bottle';
    default:
      return `${number} bottles`;
  }
}

function action(currVerse) {
  if (currVerse === 0) {
    return 'Go to the store and buy some more, ';
  }

  let sbj = currVerse === 1 ? 'it' : 'one';
  return 'Take ' + sbj + ' down and pass it around, ';
}

function nextBottle(currVerse) {
  let number = currVerse === 0 ? 99 : currVerse - 1;
  return bottles(number).toLowerCase() + ' of beer on the wall.\n';
}
