package strand

import "strings"

const testVersion = 3

func ToRNA(dna string) (rna string) {
	strands := strings.Split(dna, "")
	babel := map[string]string{
		"G": "C",
		"C": "G",
		"T": "A",
		"A": "U",
	}

	for _, s := range strands {
		r := babel[s]
		rna += r
	}

	return
}
