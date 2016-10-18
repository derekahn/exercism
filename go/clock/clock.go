package clock

import "fmt"

const (
	testVersion = 4
	aDay        = 24
	anHour      = 60
)

type Clock int

func New(hour, minutes int) Clock {
	time := (hour*anHour + minutes) % (anHour * aDay)
	if time < 0 {
		time += anHour * aDay
	}

	return Clock(time)
}

func (c Clock) String() string {
	return fmt.Sprintf("%02d:%02d", c/anHour, c%anHour)
}

func (c Clock) Add(minutes int) Clock {
	return New(0, int(c)+minutes)
}
