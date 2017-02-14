export default class Transcriptor {
  constructor(G = 'C', C = 'G', T = 'A', A = 'U') {
    this.decode = {G, C, T, A};
  }

  toRna(dna) {
    let rna = '';

    dna.split('').forEach(strand => {
      let decoded = this.decode[strand];

      if (decoded) {
        rna += decoded;
      } else {
        throw new Error('Invalid input DNA.');
      }
    });

    return rna;
  }
};
