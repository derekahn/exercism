package slice

func All(n int, s string) (combos []string) {
	for i := 0; n <= len(s); i++ {
		combos = append(combos, s[i:n])
		n++
	}
	return
}

func UnsafeFirst(n int, s string) string {
	return s[:n]
}
