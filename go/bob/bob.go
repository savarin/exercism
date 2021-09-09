package bob

import "strings"

func Hey(remark string) string {
	n := len(remark)

	if remark[n - 1] == '?' {
		if remark == strings.ToUpper(remark) {
			return "Calm down, I know what I'm doing!"
		}

		return "Sure."
	}

	if remark == strings.ToUpper(remark) {
		return "Whoa, chill out!"
	}

	if remark == "" {
		return "Fine. Be that way!"
	}

	return "Whatever."
}
