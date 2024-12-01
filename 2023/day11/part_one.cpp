#include <algorithm>
#include <fstream>
#include <iostream>
#include <vector>

// ------------------------------------------------------------------------------------------ Display ---
void display_space(const std::vector<std::vector<char>> &space) {
  for (const auto &line : space) {
    for (const auto &c : line) {

      if (c == '.')
        std::cout << "\033[1;36m.\033[0m";
      else
        std::cout << "\033[1;33m" << c << "\033[0m";
    }
    std::cout << "\n";
  }
}

// -------------------------------------------------------------------------------------- Next galaxy ---
int next_galaxy(std::vector<std::vector<char>> &space, int row, int column, bool start = false) {

  int source[2] = {row, column - 1};

  int total = 0;
  for (; row < space.size(); ++row) {
    if (row > source[0])
      column = 0;

    for (; column < space[0].size(); ++column) {
      if (space[row][column] != '.') {

        if (space[row][column] != 'X')
          total += next_galaxy(space, row, column + 1);

        if (start)
          return total;

        else {
          total += row - source[0] + std::abs(column - source[1]);
          space[row][column] = 'X';
        }
      }
    }
  }
  return total;
}

// --------------------------------------------------------------------------------------------- Main ---
int main(int argc, char *argv[]) {

  std::string line;
  std::ifstream file("input.txt");

  std::vector<std::vector<char>> space;

  if (!file.is_open()) {
    std::cout << "Unable to open file";
    return 1;
  } else {

    while (getline(file, line)) {
      space.push_back({});
      for (auto &c : line)
        space.back().push_back(c);
    }

    file.close();
  }

  // ----------------------------------------------------------------------------------------- Expend ---
  // Double empty columns
  for (int column = 0; column < space[0].size(); ++column) {

    bool ok = true;
    for (int row = 0; row < space.size(); ++row) {
      if (space[row][column] != '.') {
        ok = false;
        break;
      }
    }

    if (ok) {
      for (int row = 0; row < space.size(); ++row)
        space[row].insert(space[row].begin() + column, '.');

      ++column;
    }
  }

  // Double empty lines
  for (auto it = space.begin(); it != space.end(); ++it) {
    if (std::find((*it).begin(), (*it).end(), '#') == (*it).end()) {
      it = space.insert(it, *it);
      ++it;
    }
  }

  // display_space(space);
  // ---------------------------------------------------------------------------------------- Process ---
  int total = 0;
  total = next_galaxy(space, 0, 3, true);

  display_space(space);

  std::cout << total << "\n";
  return 0;
}
