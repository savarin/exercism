package isogram

import "unicode"

func IsIsogram(word string) bool {
	chars := make(map[rune]int)

	for i := 0; i < len(word); i++ {
		char := unicode.ToUpper(rune(word[i]))
		_, ok := chars[char]

		if ok {
			return false
		}

		if char != '-' && char != ' ' {
			chars[char] += 1
		}

	}

	return true
}
