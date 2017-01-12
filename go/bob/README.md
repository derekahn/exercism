# Bob

Bob is a lackadaisical teenager. In conversation, his responses are very limited.

Bob answers 'Sure.' if you ask him a question.

He answers 'Whoa, chill out!' if you yell at him.

He says 'Fine. Be that way!' if you address him without actually saying
anything.

He answers 'Whatever.' to anything else.

To run the tests simply run the command `go test` in the exercise directory.

If the test suite contains benchmarks, you can run these with the `-bench`
flag:

    go test -bench .

For more detailed info about the Go track see the [help
page](http://exercism.io/languages/go).

## Source

Inspired by the 'Deaf Grandma' exercise in Chris Pine's Learn to Program tutorial. [http://pine.fm/LearnToProgram/?Chapter=06](http://pine.fm/LearnToProgram/?Chapter=06)

## Submitting Incomplete Problems
It's possible to submit an incomplete solution so you can see how others have completed the exercise.

### First attempt

```go
func Hey(statement string) string {
	arr := strings.Split(statement, "")
	lastChar := []rune(arr[len(arr)-1])[0]
	second := []rune(arr[2])[0]

	response := map[rune]string{
		32:   "Fine. Be that way!", // " "
		63:   "Sure.",              // ?
		9:    "Fine. Be that way!", // \t
		8194: "Fine. Be that way!", // \u2002
	}[lastChar]

	if strings.ToUpper(statement) == statement && second > 64 && second < 91 {
		response = "Whoa, chill out!"
	}
	if response == "" {
		response = "Whatever."
	}

	return response
}
```
