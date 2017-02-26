export default class PhoneNumber {
  constructor(digits) {
    this.digits = digits;
  }

  _scrubNumber() {
    let num = this.digits.replace(/\D/g,'');

    if (num.length === 10) {
      return num;
    }

    if (num.length === 11 && num[0] === '1') {
      return num.substr(1);
    }

    return null;
  }

  number() {
    const inValid = !/^[0-9\s\-\(),.]*$/.test(this.digits);

    if (inValid) {
      return null;
    }

    return this._scrubNumber();
  }
}
