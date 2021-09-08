package hamming

import "fmt"

func Distance(a, b string) (int, error) {
	if len(a) != len(b) {
		return 0, fmt.Errorf("error")
	}

	counter := 0

	for i := 0; i < len(a); i++ {
		if a[i] != b[i] {
			counter += 1
		}
	}

	return counter, nil
}
