export default class Allergies {
  constructor(allergen) {
    this.allergen = allergen;
  }

  list() {
    const possibleAllergies = [
      'eggs',
      'peanuts',
      'shellfish',
      'strawberries',
      'tomatoes',
      'chocolate',
      'pollen',
      'cats'
    ];

    return possibleAllergies.filter((allergy, i) => {
      return this.allergen & Math.pow(2, i);
    });
  }

  allergicTo(food) {
    return this.list().some(allergy => allergy === food);
  }
}
