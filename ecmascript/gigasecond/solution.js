'use strict';

export default class Gigasecond {

  constructor(date) {
    this.time = date.getTime();
  }

  date() {
    const gigaSecond  = 1e9;
    const milliSecond = 1000;

    return new Date(this.time + (gigaSecond * milliSecond));
  }
}
