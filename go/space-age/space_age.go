package space

type Planet string

var orbitRatio = map[Planet]float64{
	Planet("Mercury"): 0.2408467,
	Planet("Venus"):   0.61519726,
	Planet("Mars"):    1.8808158,
	Planet("Jupiter"): 11.862615,
	Planet("Saturn"):  29.447498,
	Planet("Uranus"):  84.016846,
	Planet("Neptune"): 164.79132,
}

func Age(age float64, planet Planet) float64 {
	ratio, ok := orbitRatio[planet]

	if ok {
		return age / (31557600.0 * ratio)
	}

	return age / 31557600.0
}
