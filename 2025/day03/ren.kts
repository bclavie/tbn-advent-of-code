import java.io.File

data class Bank(val joltages: List<Int>) {
    fun findBankJoltage(batteryCount: Int): Long {
        var result = ""
        var pos = 0
        while (result.length < batteryCount) {
            val windowSize = joltages.size - pos - (batteryCount - result.length) + 1
            var maxDigit = 0
            var maxPos = pos
            for (i in pos until pos + windowSize) {
                if (joltages[i] > maxDigit) {
                    maxDigit = joltages[i]
                    maxPos = i
                }
            }

            result += maxDigit
            pos = maxPos + 1
        }
        return result.toLong()
    }
}

val totalJoltage = File("./input.txt").readLines()
    .filterNot(CharSequence::isBlank)
    .map { bank -> Bank(bank.map { char -> char.toString().toInt() }) }
    .sumOf { bank -> bank.findBankJoltage(12) }

println("total joltage: $totalJoltage")
