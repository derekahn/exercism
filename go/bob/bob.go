package bob

import (
	"regexp"
	"strings"
)

const testVersion = 2

func Hey(said string) string {

	isQuestion := regexp.MustCompile(`\?\s*$`)
	hasWord := regexp.MustCompile(`\w`)

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
