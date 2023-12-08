#include <array>
#include <fstream>
#include <iostream>
#include <map>
#include <vector>

typedef struct {
  std::string start;
  int64_t value;
} Step;

// ------------------------------------------------------------------------------------------------- Main ---
int main(int argc, char *argv[]) {

  std::string line;
  std::ifstream file("input.txt");

  std::string instructions;
  std::map<std::string, std::array<std::string, 2>> nodes;

  if (!file.is_open()) {
    std::cout << "Unable to open file";
    return 1;
  } else {

    while (getline(file, line)) {
      if (!line.empty()) {
        if (instructions.empty()) {
          instructions = line;

        } else
          nodes[line.substr(0, 3)] = {line.substr(7, 3), line.substr(12, 3)};
      }
    }

    file.close();
  }

  // ------------------------------------------------------------------------------ Move number per start ---
  // Take all starts
  std::vector<Step> steps;
  for (const auto &n : nodes) {
    if (n.first[2] == 'A')
      steps.push_back({.start = n.first, .value = 0});
  }

  for (auto &s : steps) {

    std::string current_value = s.start;
    int moves = 0;
    int i = 0;
    while (true) {
      ++moves;

      if (instructions[i] == 'R')
        current_value = nodes[current_value][1];
      else
        current_value = nodes[current_value][0];

      if (current_value[2] == 'Z') {
        s.value = moves;
        std::cout << moves << " moves for " << s.start << "\n";
        break;
      }

      if (i == instructions.size() - 1)
        i = 0;
      else
        i++;
    }
  }

  // ------------------------------------------------------------------------------------------------ LCM ---
  // instructions.size() is the greatest common divisor because each path is n loops of complete instructions

  int64_t lcm = steps[0].value; // PPCM in french
  for (int i = 1; i < steps.size(); ++i)
    lcm *= steps[i].value / instructions.size();

  std::cout << "\nTotal " << lcm << "\n";
  return 0;
}
