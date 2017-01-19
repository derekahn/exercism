package bob

import (
	"regexp"
	"strings"
)

const testVersion = 2

var isQuestion = regexp.MustCompile(`\?\s*$`)
var hasWord = regexp.MustCompile(`\w`)

func Hey(said string) string {
	if said != strings.ToLower(said) && said == strings.ToUpper(said) {
		return "Whoa, chill out!"
	}
	if isQuestion.MatchString(said) {
		return "Sure."
	}
	if !(hasWord.MatchString(said)) {
		return "Fine. Be that way!"
	}

	return "Whatever."
}
