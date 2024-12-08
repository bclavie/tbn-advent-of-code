#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdbool.h>
#include <assert.h>

static char *read_file_to_str(const char *path)
{
    FILE *fd = fopen(path, "r");
    if (!fd) {
        perror("failed to open file");
        return NULL;
    }

    if (fseek(fd, 0, SEEK_END) != 0) {
        perror("failed to seek in file");
        fclose(fd);
        return NULL;
    }

    size_t sz = ftell(fd);

    if (sz == (size_t)-1) {
        perror("failed to tell file size");
        fclose(fd);
        return NULL;
    }

    rewind(fd);

    char *mem = malloc(sz + 1);
    if (!mem) {
        perror("failed to allocate memory");
        fclose(fd);
        return NULL;
    }

    size_t sz_read = fread(mem, 1, sz, fd);
    if (sz_read != sz) {
        perror("failed to read file contents");
        free(mem);
        fclose(fd);
        return NULL;
    }

    // null-terminate the string
    mem[sz] = '\0';
    fclose(fd);
    return mem;
}

static size_t split_by(char *input, const char *delim, char ***out_list)
{
    assert(input != NULL);
    assert(delim != NULL);
    assert(out_list != NULL);

    char **list = NULL;
    size_t count = 0;
    char *saveptr;
    char *token = strtok_r(input, delim, &saveptr);

    while (token != NULL) {
        char **new_list = realloc(list, (count + 1) * sizeof(*list));
        if (!new_list) {
            perror("failed to allocate memory for token list");
            free(list);
            return 0;
        }
        list = new_list;
        list[count++] = token;
        token = strtok_r(NULL, delim, &saveptr);
    }

    *out_list = list;
    return count;
}

typedef struct cell {
    bool obstructed;
    int times_visited;
} cell_t;

typedef struct grid {
    cell_t *cells;
    size_t width;
    size_t height;
} grid_t;

typedef enum {
    UP,
    RIGHT,
    DOWN,
    LEFT,
} direction_t;

typedef struct position {
    int x, y;
} position_t;

typedef struct guard {
    position_t position;
    direction_t direction;
} guard_t;

static position_t next_guard_pos(guard_t *guard)
{
    position_t pos = guard->position;

    switch (guard->direction) {
        case UP:
            pos.y -= 1;
            break;

        case DOWN:
            pos.y += 1;
            break;

        case LEFT:
            pos.x -= 1;
            break;

        case RIGHT:
            pos.x += 1;
            break;
    }

    return pos;
}

static bool is_grid_pos(grid_t *grid, position_t *pos)
{
    return pos->x >= 0 && pos->x < grid->width && pos->y >= 0 && pos->y < grid->height;
}

static void part1(grid_t *grid, guard_t _guard)
{
    int total = 0;
    guard_t *guard = &_guard;
    position_t *pos = &guard->position;

    while (is_grid_pos(grid, pos)) {
        cell_t *cell = &grid->cells[pos->y * grid->width + pos->x];

        if (!cell->times_visited) {
            total += 1;
        }

        cell->times_visited += 1;

        while (1) {
            position_t next_pos = next_guard_pos(guard);

            if (!is_grid_pos(grid, &next_pos)) {
                *pos = next_pos;
                break;
            }

            cell_t *next_cell = &grid->cells[next_pos.y * grid->width + next_pos.x];
            
            if (!next_cell->obstructed) {
                *pos = next_pos;
                break;
            }

            guard->direction += 1;
            guard->direction %= 4;
        }
    }

    printf("%i\n", total);
}

static void part2(grid_t *grid, guard_t _guard)
{
    int total = 0;

    // create a graph with nodes (position, direction). compute
    // the path the guard takes through these nodes. add obstructions,
    // detect if they create a cycle.

    // or this...
    guard_t *guard = &_guard;
    guard_t original_guard = *guard;

    // reset grid
    for (int y2 = 0; y2 < grid->height; ++y2) {
        for (int x2 = 0; x2 < grid->width; ++x2) {
            grid->cells[y2 * grid->width + x2].times_visited = 0;
        }
    }

    for (int y = 0; y < grid->height; ++y) {
        for (int x = 0; x < grid->width; ++x) {
            if (guard->position.x == x && guard->position.y == y) {
                continue;
            }

            cell_t *selected_cell = &grid->cells[y * grid->width + x];

            if (selected_cell->obstructed) {
                continue;
            }

            selected_cell->obstructed = true;

            position_t *pos = &guard->position;
            bool looped = false;

            while (!looped && is_grid_pos(grid, pos)) {
                cell_t *cell = &grid->cells[pos->y * grid->width + pos->x];

                while (1) {
                    position_t next_pos = next_guard_pos(guard);

                    if (!is_grid_pos(grid, &next_pos)) {
                        *pos = next_pos;
                        break;
                    }

                    cell_t *next_cell = &grid->cells[next_pos.y * grid->width + next_pos.x];
                    
                    if (!next_cell->obstructed) {
                        *pos = next_pos;
                        break;
                    }

                    if (cell->times_visited & (1 << guard->direction)) {
                        looped = true;
                        break;
                    }

                    cell->times_visited |= (1 << guard->direction);
                    
                    guard->direction += 1;
                    guard->direction %= 4;
                }
            }

            if (looped) {
                total += 1;
            }

            // reset grid
            for (int y2 = 0; y2 < grid->height; ++y2) {
                for (int x2 = 0; x2 < grid->width; ++x2) {
                    grid->cells[y2 * grid->width + x2].times_visited = 0;
                }
            }

            // reset guard
            *guard = original_guard;
            selected_cell->obstructed = false;
        }
    }

    printf("%i\n", total);
}

int main(int argc, char *argv[])
{
    char *input = read_file_to_str("input/day06.txt");

    char **grid_lines = NULL;
    size_t height = split_by(input, "\r\n", &grid_lines);
    assert(height > 0);

    // calculate the number of columns in this grid
    size_t width = strlen(grid_lines[0]);
    assert(width > 0);

    // allocate memory for the grid
    grid_t grid;
    grid.cells = calloc(width * height, sizeof(cell_t));
    grid.width = width;
    grid.height = height;

    guard_t guard;

    for (int i = 0; i < grid.height; ++i) {
        assert(strlen(grid_lines[i]) == grid.width);

        for (int j = 0; j < grid.width; ++j) {
            cell_t *cell = &grid.cells[i * grid.width + j];

            switch (grid_lines[i][j]) {
                case '#':
                    cell->obstructed = true;
                    break;
                case '^':
                    guard.direction = UP;
                    guard.position.x = j;
                    guard.position.y = i;
                    break;
                case 'v':
                    guard.direction = DOWN;
                    guard.position.x = j;
                    guard.position.y = i;
                    break;
                case '<':
                    guard.direction = LEFT;
                    guard.position.x = j;
                    guard.position.y = i;
                    break;
                case '>':
                    guard.direction = RIGHT;
                    guard.position.x = j;
                    guard.position.y = i;
                    break;
                default:
                    break;
            }
        }
    }

    part1(&grid, guard);
    part2(&grid, guard);
    return 0;
}