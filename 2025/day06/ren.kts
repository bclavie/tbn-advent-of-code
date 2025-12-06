import java.io.File
import kotlin.times

data class Calculation(val operation: Char, val operands: List<Long>) {
    fun perform() = when (operation) {
        '+' -> operands.sum()
        '*' -> operands.reduce { a, b -> a * b }
        else -> operands.sum()
    }
}

val lines = File("./input.txt").readLines()
val columns = lines.flatMap { it.withIndex() }
    .groupBy { it.index }
    .map { group -> group.value.map { it.value } }
val calculations = columns.fold(emptyList<Calculation>()) { acc, next ->
    if (next.joinToString("").isBlank()) return@fold acc
    val operation = next.last()
    if (operation == '+' || operation == '*') {
        return@fold acc + Calculation(
            operation,
            listOf(next.dropLast(1).joinToString("").trim().toLong())
        )
    } else {
        return@fold acc.dropLast(1) + acc.last().copy(
            operands = acc.last().operands + next.dropLast(1).joinToString("").trim().toLong()
        )
    }
}
val answer = calculations.sumOf(Calculation::perform)

println("answer: $answer")
