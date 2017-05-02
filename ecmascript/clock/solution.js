export default (hour, minute = 0) => {
  const clock = { hour, minute };

  const adjustTime = (d = 0) => {
    const dayIn = { mins: 1440, hours: 24 };
    const delta = Math.abs(d) >= dayIn.mins ? d % dayIn.mins : d;

    const currentMinutes = clock.hour * 60 + clock.minute;
    let newMinutes = (currentMinutes + delta) % dayIn.mins;

    newMinutes = newMinutes < 0 ? (newMinutes += dayIn.mins) : newMinutes;

    clock.hour = Math.floor(newMinutes / 60) % dayIn.hours;
    clock.minute = newMinutes - clock.hour * 60;
  };

  adjustTime();

  return {
    clock,
    toString() {
      const formatNum = num => {
        const numString = num.toString();
        return numString.length === 1 ? `0${numString}` : numString;
      };
      return `${formatNum(clock.hour)}:${formatNum(clock.minute)}`;
    },
    plus(minutes) {
      adjustTime(minutes);
      return this;
    },
    minus(minutes) {
      adjustTime(-minutes);
      return this;
    },
    equals(otherClock) {
      return (
        clock.hour === otherClock.clock.hour &&
        clock.minute === otherClock.clock.minute
      );
    },
  };
};
