package anagram

import (
	"sort"
	"strings"
)

const testVersion = 1

func Detect(word string, list []string) (anagrams []string) {
	list = filterList(list, word)
	word = normalize(word)

	m := formatList(list)
	for k, v := range m {
		if v == word {
			anagrams = append(anagrams, k)
		}
	}
	return
}

func filterList(list []string, word string) (filtered []string) {
	for _, w := range list {
		if len(w) == len(word) && strings.ToLower(w) != strings.ToLower(word) {
			filtered = append(filtered, w)
		}
	}
	return
}

func formatList(list []string) map[string]string {
	m := make(map[string]string)
	for _, w := range list {
		key := strings.ToLower(w)
		val := normalize(key)
		m[key] = val
	}
	return m
}

func normalize(word string) string {
	word = strings.ToLower(word)
	chars := strings.Split(word, "")
	sort.Strings(chars)
	return strings.Join(chars, "")
}
