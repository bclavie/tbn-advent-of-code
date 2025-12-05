import java.io.File
import kotlin.math.max
import kotlin.math.min

val input = File("./input.txt").readLines()
val ranges = input.takeWhile(CharSequence::isNotBlank).map { range ->
    val (start, end) = range.split("-").map(String::toLong)
    LongRange(start, end)
}

val availableIngredients = input.dropWhile(CharSequence::isNotBlank)
    .filter(CharSequence::isNotBlank)
    .map(String::toLong)

val freshIngredients = availableIngredients.filter { ingredient ->
    ranges.any { range -> range.contains(ingredient) }
}

val sorted = ranges.sortedBy { it.first }
val merged = mutableListOf<LongRange>()
var current = sorted.first()

for (range in sorted.drop(1)) {
    if (range.first <= current.last + 1) {
        current = current.first..maxOf(current.last, range.last)
    } else {
        merged.add(current)
        current = range
    }
}
merged.add(current)

val possibleFreshIngredients = merged.sumOf { range -> range.last - range.first + 1}

println("fresh ingredients: ${freshIngredients.size}")
println("possible fresh ingredients: $possibleFreshIngredients")