#include <fstream>
#include <iostream>
#include <vector>

const char OK = 'I';
const char PIPE = '0';
const char OUTSIDE = 'X';
const char START = 'S';

// ------------------------------------------------------------------------------------------ Display ---
void display_grid(std::vector<std::vector<char>> &grid) {

  for (size_t i = 0; i < grid.size(); i += 2) {
    for (size_t j = 0; j < grid[i].size(); j += 2) {

      if (grid[i][j] == OK)
        std::cout << "\033[1;31m" << OK << "\033[0m";
      else if (grid[i][j] == PIPE)
        std::cout << "\033[1;36m" << PIPE << "\033[0m";
      else if (grid[i][j] == START)
        std::cout << "\033[1;33m" << START << "\033[0m";
      else
        std::cout << "\033[1;30m" << OUTSIDE << "\033[0m";
    }
    std::cout << "\n";
  }
}

// --------------------------------------------------------------------------------------------- Next ---
// Follow the pipe up to to start and replace char by PIPE
int next(std::vector<std::vector<char>> &grid, size_t i, size_t j, int nb_step) {

  char north = (i > 0) ? grid[i - 1][j] : 'X';
  char east = (j < grid[0].size() - 1) ? grid[i][j + 1] : 'X';
  char south = (i < grid.size() - 1) ? grid[i + 1][j] : 'X';
  char west = (j > 0) ? grid[i][j - 1] : 'X';

  ++nb_step;
  if (grid[i][j] == START && nb_step > 2)
    return nb_step;

  char val = grid[i][j];
  if (val != START)
    grid[i][j] = PIPE;

  switch (val) {

  case '|':
    if (north == '|' || north == 'F' || north == '7' || (north == START && nb_step > 2))
      return next(grid, i - 1, j, nb_step);
    else
      return next(grid, i + 1, j, nb_step);

  case '-':
    if (east == '-' || east == '7' || east == 'J' || (east == START && nb_step > 2))
      return next(grid, i, j + 1, nb_step);
    else
      return next(grid, i, j - 1, nb_step);

  case 'L':
    if (north == '|' || north == 'F' || north == '7' || (north == START && nb_step > 2))
      return next(grid, i - 1, j, nb_step);
    else
      return next(grid, i, j + 1, nb_step);

  case 'J':
    if (north == '|' || north == 'F' || north == '7' || (north == START && nb_step > 2))
      return next(grid, i - 1, j, nb_step);
    else
      return next(grid, i, j - 1, nb_step);

  case '7':
    if (south == '|' || south == 'L' || south == 'J' || (south == START && nb_step > 2))
      return next(grid, i + 1, j, nb_step);
    else
      return next(grid, i, j - 1, nb_step);

  case 'F':
    if (south == '|' || south == 'L' || south == 'J' || (south == START && nb_step > 2))
      return next(grid, i + 1, j, nb_step);
    else
      return next(grid, i, j + 1, nb_step);

  case START:

    // Search +1 only to start to avoid the streched lines
    north = (i > 1) ? grid[i - 2][j] : 'X';
    east = (j < grid[0].size() - 2) ? grid[i][j + 2] : 'X';
    south = (i < grid.size() - 2) ? grid[i + 2][j] : 'X';
    west = (j > 1) ? grid[i][j - 2] : 'X';

    if (north == '|' || north == 'J' || north == 'F')
      return next(grid, i - 1, j, nb_step);
    else if (east == '-' || east == 'J' || east == '7')
      return next(grid, i, j + 1, nb_step);
    else if (south == '|' || south == 'J' || south == 'L')
      return next(grid, i + 1, j, nb_step);
    else if (east == '-' || east == 'J' || east == '7')
      return next(grid, i, j + 1, nb_step);
    else
      return next(grid, i, j - 1, nb_step);
  }

  return 0;
}

// -------------------------------------------------------------------------------------------- Clean ---
// Search and replace everything which is contiguous and not a PIPE
void clean(std::vector<std::vector<char>> &grid, size_t i, size_t j) {

  if (grid[i][j] != PIPE && grid[i][j] != OUTSIDE && grid[i][j] != START) {

    char north = (i > 0) ? grid[i - 1][j] : OUTSIDE;
    char east = (j < grid[0].size() - 1) ? grid[i][j + 1] : OUTSIDE;
    char south = (i < grid.size() - 1) ? grid[i + 1][j] : OUTSIDE;
    char west = (j > 0) ? grid[i][j - 1] : OUTSIDE;

    grid[i][j] = OUTSIDE;

    if (north != PIPE && north != OUTSIDE)
      clean(grid, i - 1, j);
    if (east != PIPE && east != OUTSIDE)
      clean(grid, i, j + 1);
    if (south != PIPE && south != OUTSIDE)
      clean(grid, i + 1, j);
    if (west != PIPE && west != OUTSIDE)
      clean(grid, i, j - 1);
  }
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
        for (const auto &c : line) {
          grid.back().emplace_back(c);
          grid.back().emplace_back('-'); // Horizontal strech ------------------
        }

        grid.push_back(std::vector(grid[0].size(), '|')); // Vertical strech ---
      }
    }

    file.close();
  }

  // -------------------------------------------------------------------------------- Follow the pipe ---
  for (size_t i = 0; i < grid.size(); ++i) {
    for (size_t j = 0; j < grid[i].size(); ++j) {
      if (grid[i][j] == START) {
        next(grid, i, j, 0);
        break;
      }
    }
  }

  // ------------------------------------------------------------------------------------------ Clean ---
  // Start remove from all borders
  for (size_t i = 0; i < grid.size(); ++i)
    clean(grid, i, 0);

  for (size_t i = 0; i < grid.size(); ++i)
    clean(grid, i, grid[i].size() - 1);

  for (size_t j = 0; j < grid[0].size(); ++j)
    clean(grid, 0, j);

  for (size_t j = 0; j < grid[0].size(); ++j)
    clean(grid, grid.size() - 1, j);

  // ------------------------------------------------------------------------------------------ Total ---
  int total = 0;
  for (size_t i = 0; i < grid.size(); i += 2) {
    for (size_t j = 0; j < grid[i].size(); j += 2) {

      if (grid[i][j] != PIPE && grid[i][j] != OUTSIDE && grid[i][j] != 'S') {
        grid[i][j] = OK;
        total++;
      }
    }
  }
  display_grid(grid);

  std::cout << total << "\n";
  return 0;
}
