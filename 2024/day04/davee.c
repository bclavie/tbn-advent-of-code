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

static int process_input(const char *input, bool do_extra_op)
{
    bool do_op = true;
    int total = 0;

    const char *p = input;

    // loop until end of string
    while (*p) {
        int a, b;
        char end;

        if (do_extra_op && strncmp(p, "do()", 4) == 0) {
            do_op = 1;
            p += 4;
        }

        else if (do_extra_op && strncmp(p, "don't()", 7) == 0) {
            do_op = 0;
            p += 7;
        }

        else if (sscanf(p, "mul(%3i,%3i%c", &a, &b, &end) == 3 && end == ')') {
            // if we're in a "do" state then add the product to the total
            if (do_op) {
                total += a * b;
            }

            // move past the closing parenthesis (we know it exists from sscanf)
            p = strchr(p + 4, ')') + 1;
        }

        else {
            p++;
        }
    }

    return total;
}

typedef struct grid {
    char **data;
    size_t row_n;
    size_t column_n;
} grid_t;

static bool check_bounds_max(int indx, size_t len, int max)
{
    return (indx + len) <= max;
}

static bool check_bounds_min(int indx, size_t len, int min)
{
    // add one because our index points to the last element
    return (indx + 1 - (int)len) >= min;
}

static bool extract_horizontal(grid_t *grid, int y, int x, char *out, size_t out_len)
{
    // --
    if (!check_bounds_max(x, out_len, grid->column_n)) {
        return false;
    }

    for (int i = 0; i < out_len; ++i) {
        out[i] = grid->data[y][x + i];
    }

    return true;
}

static bool extract_vertical(grid_t *grid, int y, int x, char *out, size_t out_len)
{
    // |
    if (!check_bounds_max(y, out_len, grid->row_n)) {
        return false;
    }

    for (int i = 0; i < out_len; ++i) {
        out[i] = grid->data[y + i][x];
    }

    return true;
}

static bool extract_diagonal_left(grid_t *grid, int y, int x, char *out, size_t out_len)
{
    // \ !
    if (!check_bounds_max(y, out_len, grid->row_n) ||
        !check_bounds_min(x, out_len, 0)) {
        return false;
    }

    for (int i = 0; i < out_len; ++i) {
        out[i] = grid->data[y + i][x - i];
    }

    return true;
}

static bool extract_diagonal_right(grid_t *grid, int y, int x, char *out, size_t out_len)
{
    // /
    if (!check_bounds_max(y, out_len, grid->row_n) ||
        !check_bounds_max(x, out_len, grid->column_n)) {
        return false;
    }

    for (int i = 0; i < out_len; ++i) {
        out[i] = grid->data[y + i][x + i];
    }

    return true;
}

static void part1(grid_t *grid)
{
    int total = 0;

    // row major is a bit weird to think about. but its cache efficient guys,
    // won't somebody think about the caches???
    for (int y = 0; y < grid->row_n; ++y) {
        for (int x = 0; x < grid->column_n; ++x) {
            char extracted[4];
            assert(sizeof(extracted) == sizeof("XMAS") - 1);

            // check every direction, for XMAS and the reverse (SAMX)
            if (extract_horizontal(grid, y, x, extracted, sizeof(extracted))) {
                total += memcmp(extracted, "XMAS", sizeof(extracted)) == 0;
                total += memcmp(extracted, "SAMX", sizeof(extracted)) == 0;
            }

            if (extract_vertical(grid, y, x, extracted, sizeof(extracted))) {
                total += memcmp(extracted, "XMAS", sizeof(extracted)) == 0;
                total += memcmp(extracted, "SAMX", sizeof(extracted)) == 0;
            }

            if (extract_diagonal_left(grid, y, x, extracted, sizeof(extracted))) {
                total += memcmp(extracted, "XMAS", sizeof(extracted)) == 0;
                total += memcmp(extracted, "SAMX", sizeof(extracted)) == 0;
            }

            if (extract_diagonal_right(grid, y, x, extracted, sizeof(extracted))) {
                total += memcmp(extracted, "XMAS", sizeof(extracted)) == 0;
                total += memcmp(extracted, "SAMX", sizeof(extracted)) == 0;
            }
        }
    }

    printf("%i\n", total);
}

static void part2(grid_t *grid)
{
    int total = 0;
    for (int y = 0; y < grid->row_n; ++y) {
        for (int x = 0; x < grid->column_n; ++x) {
            char extracted[3];
            assert(sizeof(extracted) == sizeof("MAS") - 1);
            int num_in_cross = 0;

            // same as part 1, but we only count if two diagonals cross
            if (extract_diagonal_right(grid, y, x, extracted, sizeof(extracted))) {
                num_in_cross += memcmp(extracted, "MAS", sizeof(extracted)) == 0;
                num_in_cross += memcmp(extracted, "SAM", sizeof(extracted)) == 0;
            }

            if (extract_diagonal_left(grid, y, x + 2, extracted, sizeof(extracted))) {
                num_in_cross += memcmp(extracted, "MAS", sizeof(extracted)) == 0;
                num_in_cross += memcmp(extracted, "SAM", sizeof(extracted)) == 0;
            }

            if (num_in_cross == 2) {
                total += 1;
            }
        }
    }

    printf("%i\n", total);
}


int main(int argc, char *argv[])
{
    char *input = read_file_to_str("input/day04.txt");

    // ROW major grid
    grid_t grid;
    grid.row_n = split_by(input, "\r\n", &grid.data);
    assert(grid.row_n > 0);

    grid.column_n = strlen(grid.data[0]);
    assert(grid.column_n > 0);

    // sanity check the assumption of all rows containing the same number of
    // columns
    for (unsigned i = 1; i < grid.row_n; ++i) {
        assert(strlen(grid.data[i]) == grid.column_n);
    }

    part1(&grid);
    part2(&grid);
    return 0;
}