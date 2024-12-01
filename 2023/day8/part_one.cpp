#include <array>
#include <cstdint>
#include <fstream>
#include <iostream>
#include <map>

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

  // --------------------------------------------------------------------------------------------- Follow ---
  std::string current_value = (*nodes.begin()).first;

  int moves = 0;
  int i = 0;
  while (true) {
    ++moves;

    if (instructions[i] == 'R')
      current_value = nodes[current_value][1];
    else
      current_value = nodes[current_value][0];

    if (current_value == "ZZZ") {
      std::cout << moves << " moves.\n";
      break;
    }

    if (i == instructions.size() - 1)
      i = 0;
    else
      i++;
  }

  return 0;
}
