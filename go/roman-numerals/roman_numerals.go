package romannumerals

import (
	"bytes"
	"fmt"
)

const testVersion = 3

type rosettaStone struct {
	arabic int
	roman  string
}

func ToRomanNumeral(number int) (string, error) {
	buffer := bytes.NewBufferString("")

	if number <= 0 || number >= 4000 {
		return "", fmt.Errorf("The number %d is undefined in the roman numeral system.", number)
	}

	mappings := []rosettaStone{
		{1000, "M"},
		{900, "CM"},
		{500, "D"},
		{400, "CD"},
		{100, "C"},
		{90, "XC"},
		{50, "L"},
		{40, "XL"},
		{10, "X"},
		{9, "IX"},
		{5, "V"},
		{4, "IV"},
		{1, "I"},
	}

	for _, m := range mappings {
		for number >= m.arabic {
			buffer.WriteString(m.roman)
			number -= m.arabic
		}
	}

	return buffer.String(), nil
}
