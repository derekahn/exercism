export default class School {
  constructor() {
    this.db = {};
  }
  roster() {
    return this.clone(this.db);
  }
  clone(db) {
    return JSON.parse(JSON.stringify(db));
  }
  grade(level) {
    let db = this.db[level];
    return db? this.clone(db.sort()) : [];
  }
  add(name, level) {
    this.db[level] = this.grade(level).concat(name).sort();
  }
}
