import java.io.File

fun Long.isValidId(): Boolean {
    val asString = toString()
    return (1..(asString.length / 2))
        .map { asString.substring(0 until it) }
        .none { substring -> substring.repeat(asString.length / substring.length) == asString }
}

val file = File("./input.txt")
val invalidIds = file.readLines()
    .flatMap { it.split(",") }
    .filterNot(CharSequence::isBlank)
    .flatMap { seq ->
        val (start, end) = seq.split("-").map(String::toLong)
        (start..end).filterNot(Long::isValidId)
    }

println("sum: ${invalidIds.sum()}")
