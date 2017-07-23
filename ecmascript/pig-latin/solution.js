export default class PigLatin {
  translate(english) {
    const LANGUAGE_RULES_REGEXP = /^([^aeiou]?qu|[^aeiou]*)(.+)/;

    return english
      .split(' ')
      .map((word) => {
        const [, beginning, ending] = word.match(LANGUAGE_RULES_REGEXP);

        if (beginning.length === 0) {
          return `${word}ay`;
        }
        return `${ending}${beginning}ay`;
      })
      .join(' ');
  }
}
