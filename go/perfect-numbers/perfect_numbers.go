package perfect

import "errors"

const testVersion = 1

type Classification int

const (
	ClassificationPerfect Classification = iota
	ClassificationAbundant
	ClassificationDeficient
)

var ErrOnlyPositive = errors.New("perfect: cannot classify zero or negative numbers")

func Classify(n uint64) (Classification, error) {
	if n < 1 {
		return 0, ErrOnlyPositive
	}

	var sum, i uint64
	for i = 1; i <= n/2; i++ {
		if n%i == 0 {
			sum += i
		}
	}

	switch {
	case sum == n:
		return ClassificationPerfect, nil
	case sum > n:
		return ClassificationAbundant, nil
	default:
		return ClassificationDeficient, nil
	}
}
