package secret

const testVersion = 1

func Handshake(secret uint) []string {
	var handshake []string

	var signals = []struct {
		bit    uint
		add    func([]string, string) []string
		action string
	}{
		{1 << 0, toAppend, "wink"},
		{1 << 1, toAppend, "double blink"},
		{1 << 2, toAppend, "close your eyes"},
		{1 << 3, toAppend, "jump"},
		{1 << 4, toReverse, ""},
	}

	for _, signal := range signals {
		if (secret & signal.bit) != 0 {
			handshake = signal.add(handshake, signal.action)
		}
	}

	return handshake
}

func toReverse(ss []string, _ string) []string {
	for i, l := 0, len(ss)-1; i < ((l + 1) / 2); i++ {
		ss[i], ss[l-i] = ss[l-i], ss[i]
	}
	return ss
}

func toAppend(ss []string, s string) []string {
	return append(ss, s)
}
