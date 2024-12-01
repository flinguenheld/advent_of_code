#include <algorithm>
#include <fstream>
#include <iostream>
#include <vector>

constexpr char ROCK = '#';
constexpr char STONE = 'O';
constexpr char EMPTY = '.';

// ------------------------------------------------------------------------------------------ Display ---
void display_vec(const std::vector<std::string> &platform) {

  for (const auto &line : platform) {
    for (const auto &c : line) {

      if (c == EMPTY)
        std::cout << "\033[1;30m" << EMPTY << "\033[0m";
      else if (c == ROCK)
        std::cout << "\033[1;31m" << ROCK << "\033[0m";
      else if (c == STONE)
        std::cout << "\033[1;34m" << STONE << "\033[0m";
      else
        std::cout << "\033[1;37m" << c << "\033[0m";
    }
    std::cout << "\n";
  }
}

// --------------------------------------------------------------------------------------- Next stone ---
int next_north(std::vector<std::string> &platform, int row, int column) {

  int empty_position = -1;
  for (int r = row; r < platform.size(); ++r) {

    // Next if rock
    if (platform[r][column] == ROCK)
      return next_north(platform, r + 1, column);

    // Search the first empty case
    if (platform[r][column] == EMPTY && empty_position == -1)
      empty_position = r;

    // Then its stone
    else if (platform[r][column] == STONE && empty_position != -1) {
      platform[r][column] = EMPTY;
      platform[empty_position][column] = STONE;
      return next_north(platform, empty_position + 1, column);
    }
  }
  return -1;
}

int next_south(std::vector<std::string> &platform, int row, int column) {

  int empty_position = -1;
  for (int r = row; r >= 0; --r) {

    if (platform[r][column] == ROCK)
      return next_south(platform, r - 1, column);

    if (platform[r][column] == EMPTY && empty_position == -1)
      empty_position = r;

    else if (platform[r][column] == STONE && empty_position != -1) {
      platform[r][column] = EMPTY;
      platform[empty_position][column] = STONE;
      return next_south(platform, empty_position - 1, column);
    }
  }
  return -1;
}
int next_west(std::vector<std::string> &platform, int row, int column) {

  int empty_position = -1;
  for (int c = column; c < platform[0].size(); ++c) {

    if (platform[row][c] == ROCK)
      return next_west(platform, row, c + 1);

    if (platform[row][c] == EMPTY && empty_position == -1)
      empty_position = c;

    else if (platform[row][c] == STONE && empty_position != -1) {
      platform[row][c] = EMPTY;
      platform[row][empty_position] = STONE;
      return next_west(platform, row, empty_position + 1);
    }
  }
  return -1;
}

int next_east(std::vector<std::string> &platform, int row, int column) {

  int empty_position = -1;
  for (int c = column; c >= 0; --c) {

    if (platform[row][c] == ROCK)
      return next_east(platform, row, c - 1);

    if (platform[row][c] == EMPTY && empty_position == -1)
      empty_position = c;

    else if (platform[row][c] == STONE && empty_position != -1) {
      platform[row][c] = EMPTY;
      platform[row][empty_position] = STONE;
      return next_east(platform, row, empty_position - 1);
    }
  }
  return -1;
}

// -------------------------------------------------------------------------------------- Calculation ---
int calculation(std::vector<std::string> &platform) {

  int64_t total = 0;
  int multiplicateur = platform.size() + 1;

  for (int row = 0; row < platform.size(); ++row)
    total += std::count(platform[row].begin(), platform[row].end(), STONE) * --multiplicateur;

  return total;
}

// --------------------------------------------------------------------------------------------- Main ---
int main(int argc, char *argv[]) {

  std::string line;
  std::ifstream file("input.txt");

  std::vector<std::string> platform;

  if (!file.is_open()) {
    std::cout << "Unable to open file";
    return 1;
  } else {

    while (getline(file, line)) {
      if (!line.empty())
        platform.push_back(std::move(line));
    }
    file.close();
  }

  // ---------------------------------------------------------------------------------------- Process ---
  int val = 0;
  for (int i = 0; i <= 1000000000; ++i) {

    val = calculation(platform);
    std::cout << " -----> Value for  " << i << " -> " << val << "\n";

    // I haven't found any logic to skip the billion of tilts yet :'(
    // All input answer iterations are included in example answer iterations...
    // example: 90 / 97 / 104 / ... / 153 / 160 / 167 / 174 / ...
    // input:        97 /             153 /             174 / 230 / 251 / 307 / 328
    if (i == 1000) {
      break;
    }

    for (int colunm = 0; colunm < platform[0].size(); ++colunm) // North
      next_north(platform, 0, colunm);

    for (int row = 0; row < platform.size(); ++row) // West
      next_west(platform, row, 0);

    for (int colunm = 0; colunm < platform[0].size(); ++colunm) // South
      next_south(platform, platform.size() - 1, colunm);

    for (int row = 0; row < platform.size(); ++row) // East
      next_east(platform, row, platform[0].size() - 1);
  }

  // display_vec(platform);
  std::cout << calculation(platform) << "\n";

  return 0;
}
