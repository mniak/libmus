package libmus

var NAMES = []string{"C", "D", "E", "F", "G", "A", "B"}

const (
	FLAT_SYMBOL         = '♭'
	SHARP_SYMBOL        = '♯'
	DOUBLE_FLAT_SYMBOL  = '𝄫'
	DOUBLE_SHARP_SYMBOL = '𝄪'
)

var SUPERSCRIPT_OCTAVES = []string{
	"\x2070", "\x00B9", "\x00B2", "\x00B3",
	"\x2074", "\x2075", "\x2076", "\x2077",
	"\x2078", "\x2079", "\x00B9\x2070",
}
