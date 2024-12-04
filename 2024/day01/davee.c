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

static int comp_int(const void *e1, const void *e2)
{
    int s1 = *(int *)e1;
    int s2 = *(int *)e2;

    if (s1 > s2) {
        return 1;
    }
    else if (s1 < s2) {
        return -1;
    }

    return 0;
}

static void part1(const int *list1, const int *list2, size_t num)
{
    int diff_sum = 0;

    // calculate the difference between sorted elements in the list
    for (unsigned i = 0; i < num; ++i) {
        diff_sum += abs(list1[i] - list2[i]);
    }

    printf("%i\n", diff_sum);
}

static void part2(const int *list1, const int *list2, size_t num)
{
    int similarity = 0;
    const int *rlist = list2;
    size_t rlist_rem = num;
    int cached_rval = -1;
    int cached_rn = 0;

    for (unsigned i = 0; i < num; ++i) {
        // if the value we have in the first list equals the cached value from
        // the second list then we can calculate the similarity contribution
        // without further processing
        if (list1[i] == cached_rval) {
            similarity += cached_rn * list1[i];
            continue;
        }

        // CACHE MISS: find if there is a value for list1, cache if there is.

        // if the second list is exhausted then similarity will always be zero
        // for all other elements in the first list. stop
        if (rlist_rem == 0) {
            break;
        }

        // advance the second list until we meet a value equal or greater than
        // the one in the first list
        while (*rlist < list1[i]) {
            rlist++;
            rlist_rem--;

            // stop if the second list is exhausted
            if (rlist_rem == 0) {
                break;
            }
        }

        // if there is no entry in the second list for the value, then there
        // is no contribution to similarity and we can advance to the next val
        if (list1[i] != *rlist) {
            continue;
        }

        // add value to cache and remove to rlist
        cached_rval = *rlist;
        cached_rn = 1;
        rlist++;
        rlist_rem--;

        // coalesce duplicates in the second list to a single multiplier
        while (rlist_rem > 0 && *rlist == cached_rval) {
            rlist++;
            rlist_rem--;
            cached_rn++;
        }
        
        similarity += cached_rn * list1[i];
    }

    printf("%i\n", similarity);
}

int main(int argc, char *argv[])
{
    char *data = read_file_to_str("input/day01.txt");

    char **elements = NULL;
    size_t num_ele = split_by(data, "\r\n ", &elements);
    assert(num_ele % 2 == 0);

    // even = list 1, odd = list 2
    int *list1 = malloc(sizeof(int) * num_ele / 2);
    int *list2 = malloc(sizeof(int) * num_ele / 2);

    for (unsigned i = 0; i < num_ele; i += 2) {
        list1[i/2] = atoi(elements[i]);
        list2[i/2] = atoi(elements[i + 1]);
    }


    qsort(list1, num_ele/2, sizeof(int), comp_int);
    qsort(list2, num_ele/2, sizeof(int), comp_int);

    part1(list1, list2, num_ele/2);
    part2(list1, list2, num_ele/2);
    return 0;
}