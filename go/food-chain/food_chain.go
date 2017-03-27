package foodchain

import (
	"fmt"
	"strings"
)

const testVersion = 3

var rhyme = map[string]string{
	"fly":    "",
	"spider": "It wriggled and jiggled and tickled inside her.\n",
	"bird":   "How absurd to swallow a bird!\n",
	"cat":    "Imagine that, to swallow a cat!\n",
	"dog":    "What a hog, to swallow a dog!\n",
	"goat":   "Just opened her throat and swallowed a goat!\n",
	"cow":    "I don't know how she swallowed a cow!\n",
}

var animals = [8]string{
	"fly",
	"spider",
	"bird",
	"cat",
	"dog",
	"goat",
	"cow",
	"horse",
}

func sing() (verses []string) {
	for i, animal := range animals {
		verse := intro(animal) + rhyme[animal] + hook(i) + end(animal)
		verses = append(verses, verse)
	}
	return
}

func Song() string {
	return strings.Join(sing(), "\n")
}

func Verse(index int) string {
	return strings.Trim(sing()[index-1], "\n")
}

func Verses(start, end int) string {
	verses := []string{}
	for i := start; i <= end; i++ {
		verses = append(verses, Verse(i)+"\n")
	}

	return strings.Trim(strings.Join(verses, "\n"), "\n")
}

// Mechanics to create the song
func intro(thing string) string {
	return fmt.Sprintf("I know an old lady who swallowed a %s.\n", thing)
}

func hook(idx int) string {

	spiderException := func(animal string) string {
		if animal == "spider" {
			return fmt.Sprintf("%s that wriggled and jiggled and tickled inside her", animal)
		}
		return animal
	}

	if idx < 1 || idx == len(animals)-1 {
		return ""
	}

	stanzas := make([]string, idx)
	for i := range stanzas {
		stanza := fmt.Sprintf("She swallowed the %s to catch the %s.\n", animals[i+1], spiderException(animals[i]))
		stanzas = append(stanzas, stanza)
	}

	// Reverse stanzas (slice of strings)
	for i, j := 0, len(stanzas)-1; i < j; i, j = i+1, j-1 {
		stanzas[i], stanzas[j] = stanzas[j], stanzas[i]
	}

	return strings.Join(stanzas, "")
}

func end(animal string) string {
	if animal == "horse" {
		return "She's dead, of course!"
	}
	return "I don't know why she swallowed the fly. Perhaps she'll die.\n"
}
