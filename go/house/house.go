package house

const testVersion = 1

var lines = []string{
	"the horse and the hound and the horn\nthat belonged to",
	"the farmer sowing his corn\nthat kept",
	"the rooster that crowed in the morn\nthat woke",
	"the priest all shaven and shorn\nthat married",
	"the man all tattered and torn\nthat kissed",
	"the maiden all forlorn\nthat milked",
	"the cow with the crumpled horn\nthat tossed",
	"the dog\nthat worried",
	"the cat\nthat killed",
	"the rat\nthat ate",
	"the malt\nthat lay in",
}

func Verse(v int) (verse string) {
	v--
	i := len(lines) - v

	verse += compose(lines[i:], "This is ")
	verse += "the house that Jack built."
	return
}

func Song() (song string) {

	for i := 0; i <= len(lines); i++ {
		song += Verse(i + 1)

		if i < len(lines) {
			song += "\n\n"
		}
	}

	return
}

func compose(lines []string, cur string) string {
	if len(lines) == 0 {
		return cur
	}

	cur += lines[0]
	cur += " "

	return compose(lines[1:], cur)
}
