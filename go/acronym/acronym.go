package acronym

import "strings"

func Abbreviate(s string) string {
	result := s[:1]
	flag := false

	for i := 1; i < len(s); i++ {
		if ( s[i] == ' ' || s[i] == '-' || s[i] == '_' ) {
			flag = true
			continue
		}

		if flag {
			result += s[i:i+1]
			flag = false
		}
	}

	return strings.ToUpper(result)
}
