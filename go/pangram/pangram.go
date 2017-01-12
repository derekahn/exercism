package pangram

import "strings"

const testVersion = 1

func IsPangram(phrase string) bool {
	found := map[rune]bool{}

	for _, r := range strings.ToLower(phrase) {
		if r >= 'a' && r <= 'z' {
			found[r] = true
		}
	}

	return len(found) == (('z' - 'a') + 1)
}
