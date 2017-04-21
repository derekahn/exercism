export default (input = {}) => {
  let output = {};

  Object.keys(input).forEach(key => {
    let items = input[key] || [];

    items.forEach(item => {
      let value = item.toLowerCase();
      output[value] = +key;
    });
  });

  return output;
};
