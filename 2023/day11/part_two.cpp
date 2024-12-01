#include <algorithm>
#include <cstdint>
#include <fstream>
#include <iostream>
#include <vector>

constexpr int64_t TIMES = 1000000;

// ------------------------------------------------------------------------------------------ Display ---
void display_space(const std::vector<std::vector<char>> &space) {
  for (const auto &line : space) {
    for (const auto &c : line) {

      if (c == '.')
        std::cout << "\033[1;30m.\033[0m";
      else if (c == 'S')
        std::cout << "\033[1;34mS\033[0m";
      else
        std::cout << "\033[1;33m" << c << "\033[0m";
    }
    std::cout << "\n";
  }
}

// -------------------------------------------------------------------------------------- Next galaxy ---
int64_t next_galaxy(std::vector<std::vector<char>> &space, int row, int column, bool start = false) {

  int64_t total = 0;
  int64_t source[2] = {row, column - 1};

  for (; row < space.size(); ++row) {
    if (row > source[0])
      column = 0;

    for (; column < space[0].size(); ++column) {
      if (space[row][column] != '.' && space[row][column] != 'S' && space[row][column] != 'A') {

        if (space[row][column] != 'X') // Avoid the same calculation twice
          total += next_galaxy(space, row, column + 1);

        if (start) // Avoid the first calculation loop  from 0
          return total;

        else {

          // Follow the path and add TIMES for each crossed 'S'
          for (int r = source[0]; r < row; ++r) {
            if (space[r][source[1]] == 'S')
              total += TIMES - 1;
          }

          if (column - source[1] > 0) {
            for (int c = source[1]; c < column; ++c) {
              if (space[row][c] == 'S')
                total += TIMES - 1;
            }
          } else {
            for (int c = source[1]; c > column; --c) {
              if (space[row][c] == 'S')
                total += TIMES - 1;
            }
          }

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
  // Replace empty column with 'S'
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
        space[row][column] = 'S';
    }
  }

  // Replace empty row with 'S'
  for (auto it = space.begin(); it != space.end(); ++it) {
    if (std::find((*it).begin(), (*it).end(), '#') == (*it).end()) {
      for (auto &c : *it)
        c = 'S';
    }
  }

  // display_space(space);
  // ---------------------------------------------------------------------------------------- Process ---
  int64_t total = 0;
  total = next_galaxy(space, 0, 3, true);

  display_space(space);

  std::cout << total << "\n";
  return 0;
}
