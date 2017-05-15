export default (word) => {
  const map = {
    1: ['A', 'E', 'I', 'O', 'U', 'L', 'N', 'R', 'S', 'T'],
    2: ['D', 'G'],
    3: ['B', 'C', 'M', 'P'],
    4: ['F', 'H', 'V', 'W', 'Y'],
    5: ['K'],
    8: ['J', 'X'],
    10: ['Q', 'Z']
  };

  return word? word.split('').reduce((score, letter) => {
    Object.keys(map).forEach(points => {
      if (map[points].includes(letter.toUpperCase())) {
        score += +points;
      }
    });

    return score;
  }, 0) : 0;
}
