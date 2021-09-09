package luhn

func Valid(s string) bool {
	var result, index, number int

	for i := len(s) - 1; i >= 0; i-- {
		char := s[i]

		switch {
		case char == ' ':
			continue
		case char >= '0' && char <= '9':
			number = int(char - '0')

			if index%2 == 1 {
				number = number * 2
			}

			if number > 9 {
				number -= 9
			}

			result += number
			index += 1
		default:
			return false
		}

	}

	return index > 1 && result%10 == 0
}
