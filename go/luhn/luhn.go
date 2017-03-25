package luhn

import (
	"regexp"
	"strconv"
	"unicode"
)

const testVersion = 2

func Valid(cardNum string) (valid bool) {
	valid = false

	if ok := hasSymbol(cardNum); ok {
		return
	}

	str := normalize(cardNum)
	if len(str) < 2 {
		return
	}

	reversedStr := reverse(str)

	nums := []string{}
	for index, r := range reversedStr {
		if isEven(index) {
			nums = append(nums, properDouble(r))
		} else {
			nums = append(nums, string(r))
		}
	}

	sum := 0
	for _, str := range nums {
		num, _ := strconv.Atoi(str)
		sum += num
	}

	return sum%10 == 0
}

func hasSymbol(str string) bool {
	for _, letter := range str {
		if unicode.IsSymbol(letter) {
			return true
		}
	}
	return false
}

func normalize(str string) string {
	return regexp.MustCompile("[^0-9-]+").ReplaceAllString(str, "")
}

func reverse(str string) string {
	runes := []rune(str)
	for i, j := 0, len(runes)-1; i < j; i, j = i+1, j-1 {
		runes[i], runes[j] = runes[j], runes[i]
	}
	return string(runes)
}

func isEven(i int) bool {
	return (i+1)%2 == 0
}

func check(d []int) int {
	for i := len(d) - 1; i >= 0; i -= 2 {
		x := 2 * d[i]
		if x > 9 {
			x -= 9
		}
		d[i] = x
	}
	s := 0
	for _, x := range d {
		s += x
	}
	return s
}

func properDouble(r rune) string {
	num, _ := strconv.Atoi(string(r))
	double := num * 2
	if double > 9 {
		double -= 9
	}
	return strconv.Itoa(double)
}
