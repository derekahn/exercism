package brackets

const testVersion = 4

type Stack string

func push(s Stack, b byte) Stack { return Stack(b) + s }
func pop(s Stack) (Stack, byte)  { return s[1:], s[0] }

func Bracket(input string) (bool, error) {
	var stack Stack

	for i := 0; i < len(input); i++ {
		var top byte

		switch in := input[i]; in {
		case '{', '[', '(':
			stack = push(stack, in)
		case '}', ']', ')':
			if len(stack) == 0 {
				return false, nil
			}

			stack, top = pop(stack)
			switch pair := string(top) + string(in); pair {
			case "{}", "[]", "()":
				continue
			default:
				return false, nil
			}
		}
	}

	return len(stack) == 0, nil
}
