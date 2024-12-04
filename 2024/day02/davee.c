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

typedef struct report {
    size_t n;
    int *levels;
} report_t;

static int next_level(const report_t *report, int index, int dampen_index)
{
    if (index == dampen_index) {
        index += 1;

        if (index >= report->n) {
            return -1;
        }
    }

    return report->levels[index];
}

static int prev_level(const report_t *report, int index, int dampen_index)
{
    if (index == dampen_index) {
        index -= 1;

        if (index < 0) {
            return -1;
        }
    }

    return report->levels[index];
}

static bool is_safe(const report_t *report, int dampen_indx)
{
    assert(report->n > 2);

    bool is_inc = true;
    bool is_dec = true;

    for (unsigned i = 1; i < report->n; ++i) {
        int nlevel = next_level(report, i, dampen_indx);
        int plevel = prev_level(report, i - 1, dampen_indx);

        // if we have an invalid level then we attempted an oob access w.r.t
        // damping access
        if (nlevel < 0 || plevel < 0) {
            continue;
        }

        // we only care for the absolute difference between levels
        int diff = abs(nlevel - plevel);

        // we require some difference (non-zero), and within range (<= 3)
        if (!(diff != 0 && diff <= 3)) {
            return false;
        }

        // the last requirement is monotonacity. we do this by invalidating
        // the other path
        if (nlevel > plevel) {
            is_dec = false;
        }
        else if (nlevel < plevel) {
            is_inc = false;
        }
    }

    // only one of these can be true by design
    assert((is_inc && is_dec) == 0);
    return is_inc || is_dec; 
}

static void part1(const report_t *reports, size_t num)
{
    int num_safe = 0;

    for (unsigned i = 0; i < num; ++i) {
        num_safe += is_safe(&reports[i], -1);
    }

    printf("%i\n", num_safe);
}

static void part2(const report_t *reports, size_t num)
{
    int num_safe = 0;

    for (unsigned i = 0; i < num; ++i) {
        int safe = is_safe(&reports[i], -1);

        if (!safe) {
            for (unsigned j = 0; j < reports[i].n && !safe; ++j) {
                safe = is_safe(&reports[i], j);
            }
        }

        num_safe += safe;
    }

    printf("%i\n", num_safe);
}

int main(int argc, char *argv[])
{
    char *data = read_file_to_str("input/day02.txt");

    char **reports_raw = NULL;
    size_t num_reports = split_by(data, "\r\n", &reports_raw);

    report_t *reports = malloc(num_reports * sizeof(report_t));

    for (unsigned i = 0; i < num_reports; ++i) {
        char **levels_raw = NULL;
        size_t num = split_by(reports_raw[i], " ", &levels_raw);
        int *levels = malloc(num * sizeof(int *));
        for (unsigned j = 0; j < num; ++j) {
            levels[j] = atoi(levels_raw[j]);
        }
        reports[i].n = num;
        reports[i].levels = levels;
    }

    part1(reports, num_reports);
    part2(reports, num_reports);
    return 0;
}