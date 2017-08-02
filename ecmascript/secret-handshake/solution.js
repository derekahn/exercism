const REVERSE  = 'REVERSE';
const COMMANDS = ['wink', 'double blink', 'close your eyes', 'jump', REVERSE];

export default function (handshake) {
  if (typeof handshake !== 'number') {
    throw new Error('Handshake must be a number');
  }

  this.commands = () => this.shakeWith;

  this.calculateHandshake = shake =>
    COMMANDS.reduce((arr, command, i) => {
      const hasCommand = shake & Math.pow(2, i);
      if (hasCommand) {
        command === REVERSE ? arr.reverse() : arr.push(command);
      }
      return arr;
    }, []);

  this.shakeWith = this.calculateHandshake(handshake);
};
