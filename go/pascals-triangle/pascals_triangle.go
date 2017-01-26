package pascal

func Triangle(depth int) (triangle [][]int) {

	triangle = make([][]int, depth)
	row := []int{1}
	triangle[0] = row

	for i := 1; i < depth; i++ {
		last := row
		row = make([]int, i+1)
		row[0] = 1
		row[i] = 1

		for j := 1; j < i; j++ {
			row[j] = last[j-1] + last[j]
		}

		triangle[i] = row
	}

	return triangle
}
