export default class Words {
  count(str) {
    const words = str
      .trim()
      .replace(/\s /g," ")
      .replace(/\t/g, " ")
      .replace(/\n/g, " ")
      .split(' ')
      .reduce((has, key) => {

        const word = key.toLowerCase();

        if (has[word] && typeof has[word] === 'number') {
          ++has[word]
        } else {
          has[word] = 1;
        }

        return has;
      }, {});

    return words;
  }
}
