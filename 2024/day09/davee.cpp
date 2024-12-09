#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include <optional>
#include <memory>
#include <list>
#include <algorithm>

struct BlockNode {
    size_t len;
    std::optional<int> id;
};

static uint64_t checksum(const std::list<BlockNode>& nodes)
{
    uint64_t csum = 0;
    uint64_t pos = 0;

    for (auto& node : nodes) {
        if (node.id) {
            csum += (node.len * (pos + (pos + node.len - 1)) / 2) * (*node.id);
        }

        pos += node.len;
    }

    return csum;
}

static void reorder_nodes(std::list<BlockNode>& nodes)
{
    auto tail = nodes.rbegin();

    while (1) {
        // find the next file
        tail = std::find_if(tail, nodes.rend(), [](auto node){
            return node.id.has_value();
        });

        if (tail == nodes.rend()) {
            break;
        }

        // try and find a left most slot that this can fit in
        auto slot = std::find_if(nodes.begin(), std::prev(tail.base()), [tail](auto node) {
            if (node.id) {
                return false;
            }

            return node.len >= tail->len;
        });

        // if there is a slot then fill it
        if (slot != std::prev(tail.base())) {
            if (slot->len == tail->len) {
                *slot = *tail;
            } else {
                nodes.insert(slot, *tail);
                slot->len -= tail->len;
            }
    
            tail->id = std::nullopt;
        }

        tail = std::next(tail);
    }
}

int main()
{
    std::ifstream file("input/day09.txt");

    if (!file.is_open()) {
        std::cerr << "Failed to open the file.\n";
        std::abort();
    }

    std::string line;
    std::getline(file, line);

    std::list<BlockNode> part1, part2;

    // decompress
    bool is_file = true;
    int file_id = 0;

    for (size_t i = 0; i < line.size(); ++i) {
        size_t len = static_cast<size_t>(line[i] - '0');

        if (is_file) {
            part1.insert(part1.end(), len, {1, file_id});
            part2.push_back({len, file_id});
            ++file_id;
        } else {
            part1.insert(part1.end(), len, {1, std::nullopt});
            part2.push_back({len, std::nullopt});
        }

        is_file = !is_file;
    }

    // same algorithm for both parts, the difference is in segmentation
    // and data prep
    reorder_nodes(part1);
    std::cout << checksum(part1) << std::endl;

    reorder_nodes(part2);
    std::cout << checksum(part2) << std::endl; 

    return 0;
}