package queenattack

import "fmt"

const testVersion = 2

func CanQueenAttack(w, b string) (attack bool, err error) {

	if err = checkSqs(w, b); err != nil {
		return false, err
	}

	// Same column or row
	if w[0] == b[0] || w[1] == b[1] {
		return true, nil
	}

	x := w[0] - b[0]
	y := w[1] - b[1]

	// Diagonal
	return x == y || x+y == 0, nil
}

func checkSqs(w, b string) (err error) {
	if err = valSq(w); err != nil {
		return err
	}
	if err = valSq(b); err != nil {
		return err
	}
	if w == b {
		return fmt.Errorf("queens on same square")
	}
	return nil
}

func valSq(s string) error {
	if len(s) != 2 ||
		s[0] < 'a' || s[0] > 'h' ||
		s[1] < '1' || s[1] > '8' {
		return fmt.Errorf("invalid square: %q", s)
	}
	return nil
}
