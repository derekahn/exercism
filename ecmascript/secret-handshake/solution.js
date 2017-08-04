export default class HandShake {
  constructor(handshake) {
    if (typeof handshake !== 'number') {
      throw new Error('handshake requires type number');
    }
    this.handshake = handshake;
    this.shakeWith = this.calculateHandshake(handshake);
  }
  commands() {
    return this.shakeWith;
  }
  calculateHandshake() {
    const REVERSE = 'REVERSE';
    const HANDSHAKE_COMMANDS = [
      'wink',
      'double blink',
      'close your eyes',
      'jump',
      REVERSE,
    ];

    return HANDSHAKE_COMMANDS.reduce((shakeWith, command, i) => {
      const handshakeHasCommand = this.handshake & Math.pow(2, i);
      if (handshakeHasCommand) {
        if (command === REVERSE) {
          shakeWith.reverse();
        } else {
          shakeWith.push(command);
        }
      }
      return shakeWith;
    }, []);
  }
}
