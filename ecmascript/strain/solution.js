export default {
  strain(array, filter, matches) {
    return array.filter( item => filter(item) === matches );
  },

  keep(array, filter) {
    return this.strain(array, filter, true);
  },

  discard(array, filter) {
    return this.strain(array, filter, false);
  }
};
