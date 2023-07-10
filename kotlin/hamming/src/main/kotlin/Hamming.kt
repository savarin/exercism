object Hamming {

    fun compute(leftStrand: String, rightStrand: String): Int {
        if (leftStrand.length != rightStrand.length) {
            throw IllegalArgumentException("Require same length arguments.")
        }

        var counter = 0

        for ((i, char) in leftStrand.withIndex()) {
            if (char != rightStrand[i]) {
                counter += 1                
            }
        }

        return counter
    }
}
