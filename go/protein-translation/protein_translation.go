package protein

const testVersion = 1

func FromCodon(s string) string {
	codon := map[string]string{
		"AUG": "Methionine",
		"UUU": "Phenylalanine",
		"UUC": "Phenylalanine",
		"UUA": "Leucine",
		"UUG": "Leucine",
		"UCU": "Serine",
		"UCC": "Serine",
		"UCA": "Serine",
		"UCG": "Serine",
		"UAU": "Tyrosine",
		"UAC": "Tyrosine",
		"UGU": "Cysteine",
		"UGC": "Cysteine",
		"UGG": "Tryptophan",
		"UAA": "STOP",
		"UAG": "STOP",
		"UGA": "STOP",
	}

	return codon[s]
}

func FromRNA(s string) (r []string) {
	for i := 0; len(s) >= i+3; i += 3 {
		p := FromCodon(s[i : i+3])
		if p == "STOP" {
			break
		}
		r = append(r, p)
	}
	return
}
