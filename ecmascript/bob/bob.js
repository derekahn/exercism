export default class Bob {
  hey(message) {
    if (message !== message.toLowerCase() &&
        message === message.toUpperCase()) {
      return 'Whoa, chill out!';
    }
    if (message.split('')[message.length - 1] === '?') {
      return 'Sure.';
    }
    if (message == '' || /^\s+$/.test(message)) {
      return 'Fine. Be that way!';
    }
    return 'Whatever.';
  }
}
