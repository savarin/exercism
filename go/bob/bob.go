package bob

import "unicode"

func Hey(remark string) string {
	var hasDigit, hasUpper, hasLower, hasQuestion bool

	for _, char := range(remark) {
		switch {
		case unicode.IsDigit(char):
			hasDigit = true
		case unicode.IsLower(char):
			hasLower = true
			hasQuestion = false
		case unicode.IsUpper(char):
			hasUpper = true
			hasQuestion = false
		case char == '?':
			hasQuestion = true
		}
	}

	switch {
	case hasQuestion && !hasLower:
		return "Calm down, I know what I'm doing!"
	case hasQuestion:
		return "Sure."
	case hasUpper && !hasLower:
		return "Whoa, chill out!"
	case !hasUpper && !hasLower &&!hasDigit:
		return "Fine. Be that way!"
	default:
		return "Whatever."
	}
}
