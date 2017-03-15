package isogram

import (
	"strings"
	"unicode"
)

const testVersion = 1

func IsIsogram(word string) bool {
	letters := map[rune]int{}

	for _, r := range strings.ToLower(word) {
		if unicode.IsLetter(r) {
			letters[r] += 1
		}
		if letters[r] > 1 {
			return false
		}
	}

	return true
}
