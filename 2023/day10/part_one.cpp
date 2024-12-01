#include <fstream>
#include <iostream>
#include <vector>

// ------------------------------------------------------------------------------------------ Display ---
void display_grid(std::vector<std::vector<char>> &grid) {
  for (const auto &v : grid) {
    for (const auto &c : v)
      std::cout << c;

    std::cout << "\n";
  }
}

// --------------------------------------------------------------------------------------------- Next ---
int next(std::vector<std::vector<char>> &grid, size_t i, size_t j, int nb_step) {

  char north = (i > 0) ? grid[i - 1][j] : 'X';
  char east = (j < grid[0].size() - 1) ? grid[i][j + 1] : 'X';
  char south = (i < grid.size() - 1) ? grid[i + 1][j] : 'X';
  char west = (j > 0) ? grid[i][j - 1] : 'X';

  ++nb_step;
  if (grid[i][j] == 'S' && nb_step > 2)
    return nb_step;

  char val = grid[i][j];
  if (val != 'S')
    grid[i][j] = '0';

  switch (val) {

  case '|':
    if (north == '|' || north == 'F' || north == '7' || (north == 'S' && nb_step > 2))
      return next(grid, i - 1, j, nb_step);
    else
      return next(grid, i + 1, j, nb_step);

  case '-':
    if (east == '-' || east == '7' || east == 'J' || (east == 'S' && nb_step > 2))
      return next(grid, i, j + 1, nb_step);
    else
      return next(grid, i, j - 1, nb_step);

  case 'L':
    if (north == '|' || north == 'F' || north == '7' || (north == 'S' && nb_step > 2))
      return next(grid, i - 1, j, nb_step);
    else
      return next(grid, i, j + 1, nb_step);

  case 'J':
    if (north == '|' || north == 'F' || north == '7' || (north == 'S' && nb_step > 2))
      return next(grid, i - 1, j, nb_step);
    else
      return next(grid, i, j - 1, nb_step);

  case '7':
    if (south == '|' || south == 'L' || south == 'J' || (south == 'S' && nb_step > 2))
      return next(grid, i + 1, j, nb_step);
    else
      return next(grid, i, j - 1, nb_step);

  case 'F':
    if (south == '|' || south == 'L' || south == 'J' || (south == 'S' && nb_step > 2))
      return next(grid, i + 1, j, nb_step);
    else
      return next(grid, i, j + 1, nb_step);

  case 'S':
    if (north == '|' || north == 'J' || north == 'F')
      return next(grid, i - 1, j, nb_step);
    else if (south == '|' || south == 'J' || south == 'L')
      return next(grid, i + 1, j, nb_step);
    else if (east == '-' || east == 'J' || east == '7')
      return next(grid, i, j + 1, nb_step);
    else
      return next(grid, i, j - 1, nb_step);
  }

  return 0;
}

// --------------------------------------------------------------------------------------------- Main ---
int main(int argc, char *argv[]) {

  std::string line;
  std::ifstream file("input.txt");

  std::vector<std::vector<char>> grid;

  if (!file.is_open()) {
    std::cout << "Unable to open file";
    return 1;
  } else {

    while (getline(file, line)) {
      if (!line.empty()) {
        grid.push_back({});
        for (const auto &c : line)
          grid.back().emplace_back(c);
      }
    }

    file.close();
  }

  // display_grid(grid);
  // ---------------------------------------------------------------------------------------- Process ---

  int total = 0;

  // Find the S and loop up to itself
  for (size_t i = 0; i < grid.size(); ++i) {
    for (size_t j = 0; j < grid[i].size(); ++j) {
      if (grid[i][j] == 'S') {

        total = next(grid, i, j, 0);
        break;
      }
    }
  }

  // display_grid(grid);
  std::cout << total / 2 << "\n";
  return 0;
}
