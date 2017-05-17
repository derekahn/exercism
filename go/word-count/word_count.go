package wordcount

import (
	"regexp"
	"strings"
)

const testVersion = 3

type Frequency map[string]int

func WordCount(phrase string) (freq Frequency) {
	freq = make(Frequency, 0)
	for _, word := range strings.Fields(normalize(phrase)) {
		word = strings.Trim(word, "'")
		freq[word]++
	}
	return
}

func normalize(phrase string) string {
	r, _ := regexp.Compile(`[^\w|']`)
	phrase = strings.ToLower(phrase)
	return r.ReplaceAllLiteralString(phrase, " ")
}
