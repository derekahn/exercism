const OPS = {
  multiplied: '*',
  minus: '-',
  plus: '+',
  divided: '/',
};

const ArgumentError = (message = 'invalid') => ({
  name: 'argument error',
  message,
});

class WordProblem {
  constructor(question) {
    this.question = this.parse(question);
    this.hasOps = question.match(new RegExp(/(plus|minus|divided by|multiplied by)+/g));
  }
  parse(question) {
    return question.replace('?', '')
      .split(' ')
      .filter(word => !isNaN(+word) || Object.keys(OPS).includes(word))
      .map(word => OPS[word] || word)
      .join('');
  }
  answer() {
    const result = eval(this.question);
    if (!result || !this.hasOps) {
      throw new ArgumentError();
    }
    return result;
  }
}


export default { WordProblem, ArgumentError };
