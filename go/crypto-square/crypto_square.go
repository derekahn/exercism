package cryptosquare

import (
	"math"
	"regexp"
	"strings"
)

const testVersion = 2

func Encode(msg string) (cipher string) {
	str := normalize(msg)
	cols := columnAmt(len(str))

	rows := make([]string, cols)
	for i, r := range str {
		rows[i%cols] += string(r)
	}

	return strings.Join(rows, " ")
}

func columnAmt(strLength int) int {
	return int(math.Ceil(math.Sqrt(float64(strLength))))
}
func normalize(str string) string {
	return strings.ToLower(regexp.MustCompile("[^a-zA-Z0-9_]+").ReplaceAllString(str, ""))
}
