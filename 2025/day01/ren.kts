import java.io.File

enum class Direction { L, R }

data class Instruction(val direction: Direction, val count: Int) {
    companion object {
        fun parse(instruction: String) = Instruction(
            Direction.valueOf(instruction.first().toString()),
            instruction.drop(1).toInt()
        )
    }
}

data class DialState(
    val min: Int = 0,
    val max: Int = 99,
    val currentValue: Int = 50,
    val zeroValues: Int = 0
) {
    private val numberCount = max - min + 1

    fun rotate(direction: Direction): DialState {
        fun wrap(value: Int) = ((value % numberCount) + numberCount) % numberCount

        val newValue = wrap(when (direction) {
            Direction.L -> currentValue - 1
            Direction.R -> currentValue + 1
        })
        return copy(
            currentValue = newValue,
            zeroValues = zeroValues + if (newValue == 0) 1 else 0
        )
    }

    fun rotate(instruction: Instruction): DialState {
        return (0 until instruction.count).fold(this) { state, _ -> state.rotate(instruction.direction) }
    }
}

fun parse(instructions: List<String>) = instructions.map(Instruction::parse)
fun parse(file: File) = parse(file.readLines().filter(CharSequence::isNotBlank))

val file = File("./input.txt")
val finalState = parse(file).fold(DialState()) { state, instruction -> state.rotate(instruction) }
println(finalState.zeroValues)
