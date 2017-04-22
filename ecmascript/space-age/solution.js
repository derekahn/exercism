export default class SpaceAge {
  constructor(seconds) {
    this.seconds = seconds;
    this.earthYears = seconds / 31557600;
  }
}

const PLANETS = {
  Mercury: 0.2408467,
  Venus: 0.61519726,
  Earth: 1,
  Mars: 1.8808158,
  Jupiter: 11.862615,
  Saturn: 29.447498,
  Uranus: 84.016846,
  Neptune: 164.79132
};

Object.keys(PLANETS).forEach(planet => {
  SpaceAge.prototype[`on${planet}`] = function(body) {
    const years = this.earthYears / PLANETS[planet];
    return +years.toFixed(2);
  };
});
