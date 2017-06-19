export const bufferFullException = () => ({
  name: 'buffer EMPTY!',
  message: `can't read from an empty buffer!`
});

export const bufferEmptyException = () => ({
  name: 'buffer FULL!',
  message: `can't write to a full buffer!`
});

export default capacity => ({
  buffer: [],
  bufferMax: capacity,

  read() {
    if (this.buffer.length === 0){
      throw bufferEmptyException();
    }
    return this.buffer.splice(0, 1)[0];
  },

  write(value) {
    if (this.buffer.length === this.bufferMax) {
      throw bufferFullException();
    }
    if (value) {
      this.buffer.push(value);
    }
  },

  forceWrite(value) {
    if (this.buffer.length === this.bufferMax) {
      this.read();
    }
    this.write(value);
  },

  clear() {
    return this.buffer = [];
  }
});
