// Package clause.
package gigasecond

import "time"

// Constant declaration.
const testVersion = 4
const gigasecond = 1e9

// API function.  It uses a type from the Go standard library.
func AddGigasecond(t time.Time) time.Time {
	return t.Add(time.Duration(gigasecond) * time.Second)
}
