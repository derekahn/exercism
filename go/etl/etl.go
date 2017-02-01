package etl

func Transform(old map[int][]string) map[string]int {
	new := make(map[string]int)

	for points, letters := range old {
		for _, letter := range letters {
			new[string(letter[0]+32)] = points
		}
	}

	return new
}
