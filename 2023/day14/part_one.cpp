#include <algorithm>
#include <fstream>
#include <iostream>
#include <string>
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
int next_stone(std::vector<std::string> &platform, int column, int row) {

  int empty_position = -1;
  for (int r = row; r < platform.size(); ++r) {

    // Next if rock
    if (platform[r][column] == ROCK)
      return next_stone(platform, column, r + 1);

    // Search the first empty case
    if (platform[r][column] == EMPTY && empty_position == -1)
      empty_position = r;

    // Then its stone
    else if (platform[r][column] == STONE && empty_position != -1) {
      platform[r][column] = EMPTY;
      platform[empty_position][column] = STONE;
      return next_stone(platform, column, empty_position + 1);
    }
  }

  return -1;
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

  for (int colunm = 0; colunm < platform[0].size(); ++colunm) {
    next_stone(platform, colunm, 0);
  }
  display_vec(platform);

  int64_t total = 0;
  int multiplicateur = platform.size() + 1;
  for (int row = 0; row < platform.size(); ++row) {
    total += std::count(platform[row].begin(), platform[row].end(), STONE) * --multiplicateur;
  }

  std::cout << total << "\n";
  return 0;
}
