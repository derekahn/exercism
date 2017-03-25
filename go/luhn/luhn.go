package luhn

import "unicode"

const testVersion = 2

func Valid(cardNums string) (valid bool) {
	if hasSymbol(cardNums) {
		return
	}

	digits := extract(cardNums)
	if len(digits) < 2 {
		return
	}

	last := len(digits) - 1
	sum := check(digits[:last]) + digits[last]
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

func extract(num string) (digit []int) {
	for _, r := range num {
		isNum := r >= '0' && r <= '9'
		if r == ' ' || isNum {
			if isNum {
				digit = append(digit, int(r-'0'))
			}
		} else {
			return []int{}
		}
	}
	return
}

func check(digits []int) (sum int) {
	for i := len(digits) - 1; i >= 0; i -= 2 {
		doubled := 2 * digits[i]
		if doubled > 9 {
			doubled -= 9
		}

		digits[i] = doubled
	}
	for _, x := range digits {
		sum += x
	}
	return
}
