package acronym

import (
	"strings"
	"unicode"
)

const testVersion = 2

func Abbreviate(phrase string) string {

	var abbr string
	for _, words := range strings.Split(phrase, " ") {
		for _, word := range splitByUpper(words) {

			if len(word) > 1 {

				moreWords := strings.Split(word, "-")
				if len(moreWords) > 1 {

					for _, w := range moreWords {
						abbr += strings.Split(w, "")[0]
					}

				} else {
					abbr += strings.Split(word, "")[0]
				}

			}
		}
	}

	return strings.ToUpper(abbr)
}

func splitByUpper(word string) []string {

	var words []string
	var l int
	for w := word; w != ""; w = w[l:] {
		l = strings.IndexFunc(w[1:], unicode.IsUpper) + 1
		if l <= 0 {
			l = len(w)
		}
		words = append(words, w[:l])
	}

	return words
}
