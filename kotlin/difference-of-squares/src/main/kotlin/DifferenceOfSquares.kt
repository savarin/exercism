class Squares(val n: Int) {

    fun sumOfSquares(): Int {
        var result = 0

        for (i in 0..this.n) {
            result += i * i
        }

        return result
    }

    fun squareOfSum(): Int {
        var result = 0

        for (i in 0..this.n) {
            result += i
        }

        return result * result
    }

    fun difference(): Int {
        return this.squareOfSum() - this.sumOfSquares()
    }
}
