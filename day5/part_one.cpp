#include <cstdint>
#include <fstream>
#include <iostream>
#include <limits>
#include <string>
#include <vector>

// -------------------------------------------------------------------------------------------Line peeler ---
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

// ------------------------------------------------------------------------------------------------- Main ---
int main(int argc, char *argv[]) {

  std::string line;
  std::ifstream file("input.txt");

  std::vector<int64_t> seeds;
  std::vector<std::vector<std::vector<int64_t>>> maps;

  int64_t final_value_part_one = std::numeric_limits<int64_t>::max();

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
  for (const auto &s : seeds) {

    int64_t current_value = s;
    for (int a = 0; a < maps.size(); ++a) {

      // Loop to find if the current value is included in tab[1]+tab[2]
      int64_t next_value = -1;
      for (int i = 0; i < maps[a].size(); ++i) {

        if (current_value >= maps[a][i][1] && current_value <= maps[a][i][1] + maps[a][i][2]) {
          next_value = current_value - maps[a][i][1] + maps[a][i][0];
          break;
        }
      }

      if (next_value > 0)
        current_value = next_value;
    }

    if (current_value < final_value_part_one) { // Keep only one
      final_value_part_one = current_value;
    }
    // std::cout << "seed: " << s << " a pour valeur suivante: " << current_value << "\n";
    // std::cout << "----------------\n";
  }

  std::cout << final_value_part_one << "\n";
  return 0;
}
