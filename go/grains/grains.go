package grains

import "fmt"

const testVersion = 1

// << is used for "times 2" and
// >> is for "divided by 2" and
// the number after it is how many times.

// So n << x is "n times 2, x times". And y >> z is "y divided by 2, z times".
// For example, 1 << 5 is "1 times 2, 5 times" or 32. And 32 >> 5 is "32 divided by 2, 5 times" or 1.

func Square(num int) (uint64, error) {
	if num < 1 || num > 64 {
		return 0, fmt.Errorf("Invalid input: %s", num)
	}

	return 1 << (uint16(num) - 1), nil
}

func Total() uint64 {
	return 1<<64 - 1
}
