#include <vector>
#include <string>
#include <iostream>
#include <fstream>
#include <ranges>
#include <functional>

struct TrailNode {
    int height;
    int x, y;
    std::vector<TrailNode*> children;
};

struct TrailMap {
    TrailNode *at(int x, int y) {
        if (x < 0 || y < 0 || x >= width || y >= height) {
            return NULL;
        }

        return &nodes[y * height + x];
    }

    int width;
    int height;
    std::vector<TrailNode> nodes;
};

TrailMap parse_map(void)
{
    std::ifstream file("input/day10.txt");

    if (!file.is_open()) {
        std::cerr << "Failed to open the file.\n";
        std::abort();
    }

    std::string line;
    TrailMap map;

    for (map.height = 0; std::getline(file, line); ++map.height) {
        for (auto i = 0; i < line.size(); ++i) {
            map.nodes.push_back({line[i] - '0', i, map.height, {}});
        }
    }

    map.width = line.size();

    for (int y = 0; y < map.height; ++y) {
        for (int x = 0; x < map.width; ++x) {
            TrailNode *node = map.at(x, y);

            // check neighbour to west
            if (TrailNode *neighbour = map.at(x - 1, y)) {
                if (node->height + 1 == neighbour->height) {
                    node->children.push_back(neighbour);
                }
            }
            // check neighbour to east
            if (TrailNode *neighbour = map.at(x + 1, y)) {
                if (node->height + 1 == neighbour->height) {
                    node->children.push_back(neighbour);
                }
            }

            // check neighbour to north
            if (TrailNode *neighbour = map.at(x, y - 1)) {
                if (node->height + 1 == neighbour->height) {
                    node->children.push_back(neighbour);
                }
            }

            // check neighbour to south
            if (TrailNode *neighbour = map.at(x, y + 1)) {
                if (node->height + 1 == neighbour->height) {
                    node->children.push_back(neighbour);
                }
            }
        }
    }

    return map;
}

void visit_all(const TrailNode *node, std::function<void(const TrailNode*)> calc)
{
    for (auto child : node->children) {
        calc(child);
        visit_all(child, calc);
    }
}

void part1(const TrailMap& map)
{
    int total = 0;

    auto is_start_node = [](auto& node) { return node.height == 0; };

    // only interested path 0 -> 9, not each individual trail to it
    std::vector<const TrailNode *> visited;

    for (auto& start : map.nodes | std::views::filter(is_start_node)) {
        visited.clear();
        visit_all(&start, [&total, &visited, &map](auto node) {
            if (node->height == 9 && std::find(visited.begin(), visited.end(), node) == visited.end()) {
                visited.push_back(node);
                total += 1;
            }
        });
    }

    std::cout << total << std::endl;
}

void part2(const TrailMap& map)
{
    int total = 0;

    auto is_start_node = [](auto& node) { return node.height == 0; };

    for (auto& start : map.nodes | std::views::filter(is_start_node)) {
        visit_all(&start, [&total, &map](auto node) {
            if (node->height == 9) {
                total += 1;
            }
        });
    }

    std::cout << total << std::endl;
}

int main(void)
{
    TrailMap map = parse_map();
    part1(map);
    part2(map);
}