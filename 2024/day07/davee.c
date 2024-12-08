#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdbool.h>
#include <assert.h>
#include <stdint.h>
#include <math.h>

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

typedef struct equation {
    uint64_t expected;
    uint64_t *params;
    size_t param_n;
} equation_t;

static void part1(equation_t *equations, size_t equation_n)
{
    uint64_t total = 0;

    for (int i = 0; i < equation_n; ++i) {
        assert(equations[i].param_n < 64);
        assert(equations[i].param_n > 0);

        // theres 2^(param_n-1) possible solutions
        // we could prune, we could recurse

        // .. or
        for (int x = 0; x < (1 << (equations[i].param_n - 1)); ++x) {
            uint64_t calc = equations[i].params[0];

            for (int p = 1; p < equations[i].param_n; ++p) {
                if (x & (1 << p)) {
                    calc *= equations[i].params[p];
                } else {
                    calc += equations[i].params[p];
                }
            }

            if (calc == equations[i].expected) {
                total += calc;
                break;
            }
        }
    }

    printf("%lli\n", total);
}

static void part2(equation_t *equations, size_t equation_n)
{
    uint64_t total = 0;

    for (int i = 0; i < equation_n; ++i) {
        assert(equations[i].param_n < 64);
        assert(equations[i].param_n > 0);

        // theres 3^(param_n -1) possible solutions
        // we could prune, we could recurse

        // .. or
        uint64_t max = pow(3, equations[i].param_n - 1);
        for (int x = 0; x < max; ++x) {
            uint64_t calc = equations[i].params[0];

            // no base2 abuse this time
            uint64_t base3 = x;

            for (int p = 1; p < equations[i].param_n; ++p) {
                int op = base3 % 3;
                base3 = base3 / 3;

                if (op == 0) {
                    calc *= equations[i].params[p];
                } else if (op == 1) {
                    calc += equations[i].params[p];
                } else {
                    uint64_t num_base10 = log10(equations[i].params[p]) + 1;
                    calc = calc * pow(10, num_base10) + equations[i].params[p];
                }

                if (calc > equations[i].expected) {
                    break;
                }
            }

            if (calc == equations[i].expected) {
                total += calc;
                break;
            }
        }
    }

    printf("%lli\n", total);
}

int main(int argc, char *argv[])
{
    char *input = read_file_to_str("input/day07.txt");

    char **lines = NULL;
    size_t lines_n = split_by(input, "\r\n", &lines);
    assert(lines_n > 0);

    equation_t *equations = malloc(lines_n * sizeof(equation_t));

    for (int i = 0; i < lines_n; ++i) {
        char *sep = strchr(lines[i], ':');
        assert(sep);

        *sep = 0;
        equations[i].expected = strtoull(lines[i], NULL, 0);
        char **params_raw;
        equations[i].param_n = split_by(sep + 1, " ", &params_raw);
        equations[i].params = malloc(equations[i].param_n * sizeof(uint64_t));
        
        for (int j = 0; j < equations[i].param_n; ++j) {
            equations[i].params[j] = strtoull(params_raw[j], NULL, 0);
        }
    }

    part1(equations, lines_n);
    part2(equations, lines_n);
    return 0;
}