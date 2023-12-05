#include <cstdint>
#include <fstream>
#include <iostream>
#include <limits>
#include <vector>

// ------------------------------------------------------------------------------------------ Line peeler ---
void fill_int_vector(std::vector<int64_t> &vec, const std::string &txt) {

  std::string nb;
  for (auto it = txt.begin(); it != txt.end(); ++it) {

    if (std::isdigit(*it))
      nb += *it;

    if ((*it == ' ' || it == txt.end() - 1) && !nb.empty()) {
      vec.emplace_back(std::stod(nb));
      nb.clear();
    }
  }
}

// -------------------------------------------------------------------------------------- Location finder ---
uint64_t find_location(uint64_t seed, const std::vector<std::vector<std::vector<int64_t>>> &maps) {

  int64_t current_value = seed;

  // Loop in each level soil/fertilizer/water....
  for (int l = 0; l < maps.size(); ++l) {

    // Loop to find if the current value is included in tab[1]+tab[2]
    int64_t next_value = -1;
    for (int i = 0; i < maps[l].size(); ++i) {

      if (current_value >= maps[l][i][1] && current_value <= maps[l][i][1] + maps[l][i][2]) {
        next_value = current_value - maps[l][i][1] + maps[l][i][0];
        break;
      }
    }

    if (next_value > 0)
      current_value = next_value;
  }

  return current_value;
}

// ------------------------------------------------------------------------------------------------- Main ---
int main(int argc, char *argv[]) {

  std::string line;
  std::ifstream file("input.txt");

  std::vector<int64_t> seeds;
  std::vector<std::vector<std::vector<int64_t>>> maps;

  int64_t final_value_part_two = std::numeric_limits<int64_t>::max();

  if (!file.is_open()) {
    std::cout << "Unable to open file";
    return 1;
  } else {

    // int current_map = -1;
    while (getline(file, line)) {

      // ------------------------------------------------------------------------------- Build the array ---
      if (!line.empty()) {

        if (seeds.empty() && line.find("seeds: ") != std::string::npos) { // seeds: 79 14 55 13
          fill_int_vector(seeds, line);

        } else if (line.find("map:") != std::string::npos) { // seed-to-soil map:
          maps.push_back({});

        } else { // 50 98 2
          std::vector<int64_t> hop;
          fill_int_vector(hop, line);
          maps.back().emplace_back(hop);
        }
      }
    }
    file.close();
  }

  // --------------------------------------------------------------------------------------- Track seeds ---
  const int jump_length = 10005;

  // Pair values: range start
  // Odd: range length
  for (int r = 0; r < seeds.size(); r += 2) {
    std::cout << "   --- New seed range: " << seeds[r] << " / " << seeds[r] + seeds[r + 1] << " ---------\n";

    // Loop in the seed range to find the smallest location
    for (int64_t current_seed = seeds[r]; current_seed < seeds[r] + seeds[r + 1]; ++current_seed) {
      uint64_t current_location = find_location(current_seed, maps);

      // Avoid a lot of location calculations with 'jumps'
      uint64_t jump = current_seed + jump_length;
      if (jump < seeds[r] + seeds[r + 1]) {

        if (find_location(jump, maps) == current_location + jump_length) {
          // std::cout << "  jump from " << m << " to " << jump << "\n";
          current_seed = jump;
        }
      }

      if (current_location < final_value_part_two) // Keep only one
        final_value_part_two = current_location;
    }
  }

  std::cout << final_value_part_two << "\n";
  return 0;
}
