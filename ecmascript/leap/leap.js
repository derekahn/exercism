const zero = a => b => a % b == 0;

export default (year) => {
  const by = zero(year);
  return  by(400) || by(4) && !by(100)
};
