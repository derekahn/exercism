const usedName = new Set();

const generateName = () => {
  const random = max => Math.floor(Math.random()*max);
  const letter = (base = 26) => 'ABCDEFGHIJKLMNOPQRSTUVWXYZ'.charAt(random(base));

  const BASE  = 10;

  let name = letter()+letter()+random(BASE)+random(BASE)+random(BASE);

  if (usedName.has(name)) {
    name = generateName();
  }
  usedName.add(name);
  return name;
};

export default class Robot {
  constructor(){
    this.robotName = generateName();
  }
  get name() {
    return this.robotName;
  }
  reset() {
    this.robotName = generateName();
  }
}
