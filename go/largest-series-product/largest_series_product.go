package lsproduct

import "fmt"

const testVersion = 4

func LargestSeriesProduct(digits string, span int) (maxProduct int64, err error) {
	nums, err := validate(digits, span)
	if err != nil {
		return
	}

	for i, last := 0, len(nums)-span+1; i < last; i++ {
		set := int64(1)
		for _, digit := range nums[i : i+span] {
			set *= digit
		}

		if set > maxProduct {
			maxProduct = set
		}
	}
	return
}

func validate(digits string, span int) (nums []int64, err error) {
	if span < 0 {
		err = fmt.Errorf("span is negative: %d", span)
		return
	}
	if len(digits) < span {
		err = fmt.Errorf("len(%s) < span: %d < %d", digits, len(digits), span)
		return
	}
	nums = make([]int64, len(digits))
	for i, r := range digits {
		if r < '0' || r > '9' {
			err = fmt.Errorf("input %q contains non-digits", digits)
			return
		}
		nums[i] = int64(r - '0')
	}
	return
}
