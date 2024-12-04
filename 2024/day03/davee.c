#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdbool.h>

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

static void part1(const char *input)
{
    printf("%i\n", process_input(input, false));
}

static void part2(const char *input)
{
    printf("%i\n", process_input(input, true));
}

int main(int argc, char *argv[])
{
    char *input = read_file_to_str("input/day03.txt");

    part1(input);
    part2(input);
    return 0;
}