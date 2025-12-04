import java.io.File

sealed interface Entity
object Paper: Entity
object Empty: Entity

data class Grid(private val rows: List<List<Entity>>) {
    companion object {
        fun parse(lines: List<String>): Grid {
            return Grid(lines.map { line -> line.map { cell ->
                when (cell) {
                    '@' -> Paper
                    else -> Empty
                }
            }})
        }
    }

    fun entityAt(x: Int, y: Int): Entity {
        if (y < 0 || y >= rows.size) return Empty
        if (x < 0 || x >= rows[y].size) return Empty
        return rows[y][x]
    }

    fun isAccessible(x: Int, y: Int): Boolean {
        val paperCount = listOf(
            entityAt(x - 1, y - 1),
            entityAt(x, y - 1),
            entityAt(x + 1, y - 1),
            entityAt(x - 1, y),
            entityAt(x + 1, y),
            entityAt(x - 1, y + 1),
            entityAt(x, y + 1),
            entityAt(x + 1, y + 1)
        ).count { it is Paper }
        return paperCount < 4
    }

    private fun findAccessible(predicate: (Entity) -> Boolean): List<Pair<Int, Int>> = (0 until rows.size).flatMap { y ->
        (0 until rows[y].size).filter { x -> predicate(entityAt(x, y)) }
            .flatMap { x -> if (isAccessible(x, y)) listOf(x to y) else emptyList() }
    }

    fun countAccessible(predicate: (Entity) -> Boolean) = findAccessible(predicate).size

    fun removeAccessible(predicate: (Entity) -> Boolean): Grid = Grid(
        (0 until rows.size).map { y ->
            (0 until rows[y].size).map { x ->
                when (entityAt(x, y)) {
                    is Empty -> Empty
                    is Paper -> if (isAccessible(x, y)) Empty else Paper
                }
            }
        }
    )
}

val grid = Grid.parse(
    File("./input.txt").readLines()
        .filter(CharSequence::isNotEmpty)
)
var accessible = grid.countAccessible { it is Paper }
println("accessible: $accessible")

var acc = grid
var removed = 0
while (accessible > 0) {
    acc = acc.removeAccessible { it is Paper }
    removed += accessible
    accessible = acc.countAccessible { it is Paper }
}

println("removable: $removed")
