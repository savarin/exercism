// Package raindrops ....
package raindrops

import "strconv"

// Convert ...
func Convert(number int) string {
	result := ""

	if number%3 == 0 {
		result += "Pling"
	}

	if number%5 == 0 {
		result += "Plang"
	}

	if number%7 == 0 {
		result += "Plong"
	}

	if result == "" {
		return strconv.Itoa(number)
	}

	return result
}
