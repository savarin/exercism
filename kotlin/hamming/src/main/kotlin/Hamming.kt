object Hamming {

    fun compute(leftStrand: String, rightStrand: String): Int {
        if (leftStrand.length != rightStrand.length) {
            throw IllegalArgumentException("left and right strands must be of equal length")
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
