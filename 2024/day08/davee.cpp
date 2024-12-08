#include <iostream>
#include <fstream>
#include <string>
#include <vector>

struct Point {
    int x, y;
};

struct Antenna {
    char freq;
    Point p;
};

struct AntennaMap {
    AntennaMap();

    int m_width;
    int m_height;
    std::vector<Antenna> m_antennas;
};

struct AntinodeMap {
    AntinodeMap(int width_, int height_) : width(width_), height(height_), map(width * height, false) {}

    bool is_in_map(Point p)
    {
        return (p.x >= 0 && p.x < width && p.y >= 0 && p.y < height);
    }

    bool add(Point p)
    {
        if (!is_in_map(p) || map[p.y*height + p.x]) {
            return false;
        }

        map[p.y*height + p.x] = true;
        return true;
    }

    int width;
    int height;
    std::vector<bool> map;
};

AntennaMap::AntennaMap(void)
{
    std::ifstream file("input/day08.txt");

    if (!file.is_open()) {
        std::cerr << "Failed to open the file.\n";
        std::abort();
    }

    std::string line;

    for (m_height = 0; std::getline(file, line); ++m_height) {
        for (auto i = 0; i < line.size(); ++i) {
            if (!std::isalnum(line[i])) {
                continue;
            }

            m_antennas.push_back({line[i], i, m_height});
        }
    }

    m_width = line.size();
}

static void part1(const AntennaMap& map)
{
    int total = 0;
    AntinodeMap antinodes(map.m_width, map.m_height);

    // calculate antinodes
    for (auto it = map.m_antennas.begin(); it != map.m_antennas.end(); ++it) {
        for (auto it_next = std::next(it); it_next != map.m_antennas.end(); ++it_next) {
            if (it->freq != it_next->freq) {
                continue;
            }

            // calculate the difference vector
            int dx = it_next->p.x - it->p.x;
            int dy = it_next->p.y - it->p.y;

            // compute the symmetric points
            int p1_x = it->p.x - dx;
            int p1_y = it->p.y - dy;
            int p2_x = it_next->p.x + dx;
            int p2_y = it_next->p.y + dy;

            total += antinodes.add({p1_x, p1_y});
            total += antinodes.add({p2_x, p2_y});
        }
    }

    std::cout << total << std::endl;
}

static void part2(AntennaMap& map)
{
    int total = 0;
    AntinodeMap antinodes(map.m_width, map.m_height);

    // calculate antinodes
    for (auto it = map.m_antennas.begin(); it != map.m_antennas.end(); ++it) {
        for (auto it_next = std::next(it); it_next != map.m_antennas.end(); ++it_next) {
            if (it->freq != it_next->freq) {
                continue;
            }

            // calculate the difference vector
            int dx = it_next->p.x - it->p.x;
            int dy = it_next->p.y - it->p.y;

            // move along the line in one direction
            Point p = it_next->p;
            while (antinodes.is_in_map(p)) {
                total += antinodes.add(p);
                p = { p.x + dx, p.y + dy };
            }

            // move along the line in the opposite direction
            p = it_next->p;
            while (antinodes.is_in_map(p)) {
                total += antinodes.add(p);
                p = { p.x - dx, p.y - dy };
            }
        }
    }

    std::cout << total << std::endl;
}

int main()
{
    AntennaMap map;

    part1(map);
    part2(map);
    return 0;
}