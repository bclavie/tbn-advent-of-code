import java.io.File


fun findBankJoltage(bank: List<Int>, batteryCount: Int): Long {
    tailrec fun select(pos: Int, remaining: Int, acc: String = ""): String {
        if (remaining == 0) return acc
        val windowSize = bank.size - pos - remaining + 1
        val window = bank.drop(pos).take(windowSize)
        val maxDigit = window.max()
        val maxPos = pos + window.indexOf(maxDigit)
        return select(maxPos + 1, remaining - 1, acc + maxDigit)
    }
    return select(0, batteryCount).toLong()
}

val totalJoltage = File("./input.txt").readLines()
    .filterNot(CharSequence::isBlank)
    .sumOf { bank -> findBankJoltage(bank.map { char -> char.toString().toInt() }, 12) }

println("total joltage: $totalJoltage")
